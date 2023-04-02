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

const SPACES: &str = "\u{20}\r\t";
const WORD_CHARS: &str = "abcdefghijklmnopqrstuvwxyz_ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "1234567890";

#[allow(clippy::print_literal)] // bug when using file!()
fn main() {
	let mut args: Vec<String> = vec![];
	// env::args() will panic if there is an invalid input
	for arg_os in args_os() {
		args.push(arg_os.to_string_lossy().to_string());
	}

	let executor_path: String = resolve_path(PathBuf::from(args.remove(0)));

	println!("[debug] executor_path = {:?}", executor_path);

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

	println!("[debug] flags = {:?}", flags);
	// #endregion flags
	
	let (file_path, file): (String, String) = read_file(args.remove(0));
	
	println!("[debug] file_path = {:?}", file_path);
	println!("[debug] args = {:?}", args);

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

	let mut tokens: Vec<(TokenType, Option<String>)> = vec![];

	#[allow(unused)]
	#[derive(PartialEq)]
	enum ExpressionStatus {
		Done,  // println('hello world');
		Maybe, // println('hello world')
		Must,  // println('hello world'
	}

	let mut expression_status: ExpressionStatus = ExpressionStatus::Done;

	// for errors
	let mut lines: Vec<&str> = vec![];

	// TODO: put it into the 'eval' function
	for (row, line) in file.lines().enumerate() {
		println!();

		let line: &str = line.trim();
		lines.push(line);

		if line.is_empty() {
			println!("[debug] lines[{}] = \"\"", row);
			continue;
		}

		println!("[debug] lines[{}] = {:?}", row, line);

		let chars: Vec<char> = line.chars().collect::<Vec<char>>();
		let mut column: usize = 0;
		let mut current: char = chars[column];

		while column < line.len() {
			println!("[debug] column = {}", column);

			if current == ';' {
				if expression_status != ExpressionStatus::Done {
					if expression_status == ExpressionStatus::Must {
						eprintln!("SyntaxError: unexpected semicolon");
						exit(1);
					}
					expression_status = ExpressionStatus::Done;
				}
				continue;
			} else if current == '#' {
				println!("[debug] comment found");
				break;
			} else if SPACES.contains(current) {
				println!("[debug] space found");
				column += 1;
				continue;
			} else if current == '\'' || current == '"' {
				let quote: char = current;
				column += 1;
				current = chars[column];
				let token_start: usize = column;
				let mut string: String = String::new();
				while current != quote {
					column += 1;
					if column >= chars.len() {
						eprintln!("SyntaxError: unclosed string\n    at {}:{}:{}", file_path, row + 1, token_start + 1);
						exit(1);
					}
					string.push(current);
					current = chars[column];
				}
				println!("[debug] tokens.push((String, {:?}))", string);
				tokens.push((TokenType::String, Some(string)));
			} else if WORD_CHARS.contains(current) {
				panic!("word");
			} else if DIGITS.contains(current) {
				let mut number: String = String::from(current);
				column += 1;
				current = chars[column];
				while DIGITS.contains(current) {
					println!("[debug] pushing {}", current);
					number.push(current);
					column += 1;
					current = chars[column];
				}
				column -= 1;
				current = chars[column];
				println!("[debug] tokens.push((Number, {}))", number);
				tokens.push((TokenType::Number, Some(number)));
			} else {
				eprintln!("SyntaxError: invalid character \"{}\"\n    at {}:{}:{}", current, file_path, row + 1, column + 1);
				exit(1);
			}
			column += 1;
		}
		if expression_status != ExpressionStatus::Must {
			tokens.push((TokenType::Eol, None));
			println!("[debug] tokens.push((Eol, None))");
		}
	}

	println!();
	println!("[debug] tokens = {:?}", tokens);
	println!();

	// for (token_type, value) in tokens {
	// }
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

fn read_file(relative: String) -> (String, String) {
	let mut path: PathBuf = PathBuf::from(&relative);
	let full_path: String = get_full_path(relative);
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