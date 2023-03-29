use std::collections::HashMap;
use std::env::{
	args_os,
	current_dir as get_current_dir
};
use std::process::exit;
use std::path::PathBuf;
use std::fs::read_to_string;

const NAME: &str = "Sunny";
const VERSION: &str = "1.0.0";

const EXTENSION: &str = "sny";

const SPACES: &str = " \r\t";
const WORD_CHARS: &str = "abcdefghijklmnopqrstuvwxyz_ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "1234567890";

fn main() {
	let mut args: Vec<String> = vec![];
	// env::args() will panic if there is an invalid input
	for arg_os in args_os() {
		args.push(arg_os.to_string_lossy().to_string());
	}

	#[allow(unused)]
	let executor_path: String = resolve_path(PathBuf::from(args.remove(0)));
	
	if args.is_empty() {
		println!("TODO: interactive mode");
		eprintln!("ArgumentError: no arguments provided");
		exit(1);
	}

	// #region flags
	let valid_flags = [
		// String::from("--eval"),
		String::from("--help"),
		String::from("--test"),
		String::from("--version")
	];

	let unique_flags = [
		String::from("--help"),
		String::from("--version")
	];

	let flag_map: HashMap<String, String> = HashMap::from([
		// (
		// 	String::from("-e"),
		// 	String::from("--eval")
		// ),
		(
			String::from("-h"),
			String::from("--help")
		),
		(
			String::from("-t"),
			String::from("--test")
		),
		(
			String::from("-v"),
			String::from("--version")
		)
	]);

	let mut flags: Vec<String> = vec![];

	for (i, mut arg) in args.clone().iter().enumerate() {
		if arg.len() < 2 || !arg.starts_with('-') {
			break;
		}
		if arg.len() == 2 {
			if !flag_map.contains_key(&args.remove(0)) {
				eprintln!("ArgumentError: unknown flag '{}'", arg);
				exit(1);
			}
			arg = &flag_map[arg];
		}
		if !valid_flags.contains(arg) {
			eprintln!("ArgumentError: unknown flag '{}'", arg);
			exit(1);
		}
		// if arg == "--eval" {
		// 	exit(0);
		// }
		if !unique_flags.contains(arg) {
			flags.push(arg.clone());
			continue;
		}
		if i != 0 {
			eprintln!("ArgumentError: unexpected flag '{}' at position {}", arg, i);
			exit(1);
		}
		if arg == "--help" {
			println!("{}", String::new()
				+ "\x1B[90m-h\x1B[0m | \x1B[90m--help   \x1B[0m    Shows this message\n"
				+ "\x1B[90m-v\x1B[0m | \x1B[90m--version\x1B[0m    Prints the current " + NAME + " version");
		} else if arg == "--version" {
			println!("{} {}", NAME, VERSION);
		} else {
			println!("Flag not implemented yet: {}", arg);
		}
		exit(0);
	}

	dbg!(&flags);
	// #endregion flags
	
	let (file_path, file): (String, String) = import(args.remove(0));
	
	dbg!(&args);
	dbg!(&file_path);

	println!();

	let mut tokens: Vec<(TokenType, Option<String>)> = vec![];

	// let mut line_index: usize = 1;
	#[allow(unused)]
	let mut expression_done: bool = true;
	
	// TODO: put it into the 'eval' function
	for (mut line_index, line) in file.lines().enumerate() {
		line_index += 1; // one-based
		let next_char = |chars: &mut std::str::Chars, column: usize| -> char {
			match chars.next() {
				None => {
					eprintln!("InternalError: failed to read the character\n    at {}:{}:{}", file_path, line_index, column);
					exit(1);
				},
				Some(c) => c
			}
		};
		let line: &str = line.trim();
		if line.is_empty() {
			println!("empty line");
			continue;
		}
		println!("line {}: {:?}", line_index, line);
		let mut column: usize = 1;
		
		if line.len() > 255 {
			eprintln!("SyntaxError: max line length\n    at {}:{}:255", file_path, line_index);
			exit(1);
		}

		let mut chars: std::str::Chars = line.chars();
		/* 'main_loop: */
		while column < line.len() {
			// let token_start: usize;
			let mut c: char = next_char(&mut chars, column);
			// println!("column {}: '{}'", column, c);
			if c == ';' {
				eprintln!("SyntaxError: semicolons are not allowed\n    at {}:{}:{}", file_path, line_index, column);
				exit(1);
			} else if c == '#' {
				// println!("Comment found, breaking the sub-loop");
				break;
			} else if SPACES.contains(c) {
				println!("[{}:{}:{}] space at {}:{}:{}", file!(), line!(), column!(), file_path, line_index, column);
				column += 1;
				continue;
			} else if c == '\'' || c == '"' {
				println!("[{}:{}:{}] {} at {}:{}:{}", file!(), line!(), column!(), c, file_path, line_index, column);
				let quote: char = c;
				c = next_char(&mut chars, column);
				let string_start: usize = column;
				column += 1;
				let mut string: String = String::new();
				while c != quote {
					println!("[{}:{}:{}] {} at {}:{}:{}", file!(), line!(), column!(), c, file_path, line_index, column);
					if column >= line.len() {
						eprintln!("SyntaxError: unclosed string\n    at {}:{}:{}", file_path, line_index, string_start);
						exit(1);
					}
					string.push(c);
					c = next_char(&mut chars, column);
					column += 1;
				}
				println!("[{}:{}:{}] {} at {}:{}:{}", file!(), line!(), column!(), c, file_path, line_index, column);
				column += 1;
				println!("tokens.push String {:?}", string);
				tokens.push((TokenType::String, Some(string)));
			} else if WORD_CHARS.contains(c) {
				panic!("word");
			} else if DIGITS.contains(c) {
				let mut number: String = String::new();
				loop {
					if !DIGITS.contains(c) {
						break;
					}
					if c == '\'' {
						c = next_char(&mut chars, column);
						column += 1;
						continue;
					}
					if c == '.' {
						while DIGITS.contains(c) {
							println!("[{}:{}:{}] {} at {}:{}:{}", file!(), line!(), column!(), c, file_path, line_index, column);
							number.push(c); 
							c = next_char(&mut chars, column);
							column += 1;
						}
					}
					println!("[{}:{}:{}] {} at {}:{}:{}", file!(), line!(), column!(), c, file_path, line_index, column);
					number.push(c);
					c = next_char(&mut chars, column);
					column += 1;
				}
				println!("tokens.push Number {}", number);
				tokens.push((TokenType::Number, Some(number)));
			} else {
				eprintln!("SyntaxError: invalid character \"{}\"\n    at {}:{}:{}", c, file_path, line_index, column);
				exit(1);
			}
			column += 1;
		}
		if expression_done {
			tokens.push((TokenType::Eol, None));
			println!("tokens.push Eol");
		}
		// line_index += 1;
	}
	println!();
	dbg!(&tokens);
	println!();
}

fn get_full_path(relative: String) -> String {
	let current_dir: PathBuf = match get_current_dir() {
		Err(error) => {
			eprintln!("InternalError: failed to get the current directory, {}", error);
			exit(1);
		}
		Ok(dir) => dir
	};
	let path: PathBuf = current_dir.join(relative);
	path.to_string_lossy().to_string()
}

fn resolve_path(path: PathBuf) -> String {
	match path.canonicalize() {
		Err(error) => {
			eprintln!("LoadError: failed to resolve the path \"{}\", {}", path.to_string_lossy(), error);
			exit(1);
		}
		Ok(full_path) => {
			if std::env::consts::FAMILY == "windows" {
				// remove the '\\?\' prefix
				full_path.to_string_lossy()[4..].to_string()
			} else {
				full_path.to_string_lossy().to_string()
			}
		}
	}
} 

fn import(relative: String) -> (String, String) {
	let mut path: PathBuf = PathBuf::from(&relative);
	let full_path = get_full_path(relative);
	if !path.exists() {
		path.set_extension(EXTENSION);
		if !path.exists() {
			eprintln!("LoadError: file \"{}\" not found", full_path);
			exit(1);
		}
	}
	if !path.is_file() {
		eprintln!("LoadError: \"{}\" is not a file", full_path);
		exit(1);
	}
	match read_to_string(&path) {
		Err(error) => {
			eprintln!("LoadError: failed to read \"{}\", {}", full_path, error);
			exit(1);
		}
		Ok(file) => (resolve_path(path), file)
	}
}

#[allow(unused)]
#[derive(PartialEq, Debug)]
enum TokenType {
	Eol,
	Semicol,
	Word,
	Number,
	String,
	Operator,
	Bracket
}