// [[autofolding="${COMM}\s*#region**"]]

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
const ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const WORD_CHARS: &str = "abcdefghijklmnopqrstuvwxyz_ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "1234567890";
const BRACKETS: &str = "(){}[]";

#[allow(clippy::print_literal)] // bug when using file!()
fn main() {
	let mut args: Vec<String> = vec![];
	// env::args() will panic when invalid input
	for arg_os in args_os() {
		args.push(arg_os.to_string_lossy().to_string());
	}

	let executor_path: String = resolve_path(PathBuf::from(args.remove(0)));

	println!("[debug] executor_path = {:?}", executor_path);

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
				eprintln!("{}: unknown flag '{}'", red("ArgumentError"), arg);
				exit(1);
			}
			arg = &(flag_map[arg]);
		}
		if !valid_flags.contains(arg) {
			eprintln!("{}: unknown flag '{}'", red("ArgumentError"), arg);
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
			eprintln!("{}: unexpected flag '{}' at position {}", red("ArgumentError"), arg, i);
			exit(1);
		}
		if arg == "--help" {
			println!("{}", String::new()
				+ "\x1B[90m-h\x1B[0m | \x1B[90m--help   \x1B[0m    Shows this message\n"
				+ "\x1B[90m-v\x1B[0m | \x1B[90m--version\x1B[0m    Prints the current " + NAME + " version");
		} else if arg == "--version" {
			println!("{} {}", NAME, VERSION);
		} else {
			println!("{}: flag '{}' not implemented yet", red("ArgumentError"), arg);
		}
		exit(0);
	}

	println!("[debug] flags = {:?}", flags);
	// #endregion flags

	if args.is_empty() {
		println!("{}: TODO interactive mode", yellow("Warning"));
		eprintln!("{}: no arguments provided", red("ArgumentError"));
		exit(1);
	}

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

	let chars: Vec<char> = file.chars().collect::<Vec<char>>();

	enum Value {
		None,
		Bool(bool),
		Num(String),
		Range(String, String),
		Str(String),
		List(Vec<Value>),
		Dict(HashMap<String, Value>)
	}

	fn eval(code: String) -> Value {
		Value::None
	}
	// for (row, line) in file.lines().enumerate() {
	// 	println!();

	// 	let line: &str = line.trim();
	// 	lines.push(line);

	// 	if line.is_empty() {
	// 		println!("[debug] lines[{}] = (empty)", row);
	// 		continue;
	// 	}

	// 	println!("[debug] lines[{}] = {:?}", row, line);

	// 	let chars: Vec<char> = line.chars().collect::<Vec<char>>();
	// 	let mut column: usize = 0;
	// 	let mut current: char;

	// 	while column < line.len() {
	// 		current = chars[column];
	// 		println!("[debug] lines[{}][{}] = {:?}", row, column, current);
	// 		if current == ';' {
	// 			if expression_status != ExpressionStatus::Done {
	// 				if expression_status == ExpressionStatus::Must {
	// 					eprintln!("{}: unexpected semicolon", red("SyntaxError"));
	// 					exit(1);
	// 				}
	// 				expression_status = ExpressionStatus::Done;
	// 			}
	// 			tokens.push((TokenType::Semicol, None));
	// 			column += 1;
	// 			continue;
	// 		}
	// 		if current == '#' {
	// 			println!("[debug] comment found");
	// 			break;
	// 		}
	// 		if SPACES.contains(current) {
	// 			println!("[debug] space found: {:?}", current);
	// 			column += 1;
	// 			continue;
	// 		}
	// 		if current == '\'' {
	// 			let token_start: usize = column;
	// 			column += 1;
	// 			current = chars[column];
	// 			let mut string: String = String::new();
	// 			while current != '\'' {
	// 				if current == '\\' {
	// 					column += 1;
	// 					if column >= chars.len() {
	// 						eprintln!("{}: cannot escape new lines", red("SyntaxError"));
	// 						exit(1);
	// 					}
	// 					current = chars[column];
	// 					println!("[debug] escaping \\{} at {}:{}:{}", current, file_path, row + 1, column);
	// 					let escape_map: HashMap<char, char> = HashMap::from([
	// 						('n', '\n'),
	// 						('t', '\t'),
	// 						('r', '\r')
	// 					]);
	// 					if escape_map.contains_key(&current) {
	// 						string.push(escape_map[&current]);
	// 						column += 1;
	// 						current = chars[column];
	// 						continue;
	// 					}
	// 					if current == 'u' {
	// 						column += 1;
	// 						current = chars[column];
	// 						if current != '{' || column >= chars.len() {
	// 							eprintln!("{}: '{{' expected", red("SyntaxError"));
	// 							exit(1);
	// 						}
	// 						column += 1;
	// 						if column >= chars.len() {
	// 							eprintln!("{}: unexpected end of line", red("SyntaxError"));
	// 							exit(1);
	// 						}
	// 						current = chars[column];
	// 						let mut unicode: String = String::new();
	// 						while current != '}' {
	// 							if !DIGITS.contains(current) && !ALPHA.contains(current) {
	// 								eprintln!("{}: invalid unicode character '{}'", red("SyntaxError"), current);
	// 								eprintln!("    at {}:{}:{}", file_path, row + 1, column + 1);
	// 								eprintln!("        {}", lines[row]);
	// 								eprintln!("        {: >1$}", "^", column + 1);
	// 								exit(1);
	// 							}
	// 							unicode.push(current);
	// 							column += 1;
	// 							if column >= chars.len() {
	// 								eprintln!("{}: unexpected end of line", red("SyntaxError"));
	// 								exit(1);
	// 							}
	// 							current = chars[column];
	// 						}
	// 						if unicode.is_empty() {
	// 							eprintln!("{}: empty unicode secuence", red("SyntaxError"));
	// 							exit(1);
	// 						}
	// 						println!("[debug] end of unicode secuence, collected \\u{{{}}}", unicode);
	// 						match u32::from_str_radix(&unicode[..], 16) {
	// 							Err(error) => {
	// 								eprintln!("InternalError: failed to parse unicode secuence '\\u{{{}}}', {}", unicode, error);
	// 								exit(1);
	// 							},
	// 							Ok(code) => {
	// 								let character: char = match char::from_u32(code) {
	// 									None => {
	// 										eprintln!("InternalError: failed to parse unicode secuence '\\u{{{}}}'", unicode);
	// 										exit(1);
	// 									},
	// 									Some(character) => character
	// 								};
	// 								dbg!(character);
	// 								println!("\u{1b}[0m");
	// 								println!("[debug] pushing char {:?}, code = {}", character, code);
	// 								string.push(character);
	// 							}
	// 						}
	// 						column += 1;
	// 						if column >= chars.len() {
	// 							eprintln!("SyntaxError: unexpected end of line");
	// 							exit(1);
	// 						}
	// 						current = chars[column];
	// 						continue;
	// 					}
	// 					// eprintln!("{}: invalid escape secuence \\{}", red("SyntaxError"), current);
	// 					// exit(1);
	// 				}
	// 				println!("[debug] pushing {}", current);
	// 				string.push(current);
	// 				column += 1;
	// 				if column >= chars.len() {
	// 					eprintln!("{}: unclosed string", red("SyntaxError"));
	// 					eprintln!("    at {}:{}:{}", file_path, row + 1, token_start + 1);
	// 					exit(1);
	// 				}
	// 				current = chars[column];
	// 			}
	// 			println!("[debug] tokens.push((String, {:?}))", string);
	// 			tokens.push((TokenType::String, Some(string)));
	// 			column += 1;
	// 			continue;
	// 		}
	// 		if current == '"' {
	// 			column += 1;
	// 			current = chars[column];
	// 			let token_start: usize = column;
	// 			let mut string: String = String::new();
	// 			while current != '"' {
	// 				column += 1;
	// 				if column >= chars.len() {
	// 					eprintln!("{}: unclosed string", red("SyntaxError"));
	// 					eprintln!("    at {}:{}:{}", file_path, row + 1, token_start + 1);
	// 					exit(1);
	// 				}
	// 				string.push(current);
	// 				current = chars[column];
	// 			}
	// 			println!("[debug] tokens.push((String, {:?}))", string);
	// 			tokens.push((TokenType::String, Some(string)));
	// 			column += 1;
	// 			continue;
	// 		}
	// 		if WORD_CHARS.contains(current) {
	// 			let mut word: String = String::new();
	// 			while WORD_CHARS.contains(current) {
	// 				word.push(current);
	// 				column += 1;
	// 				if column >= chars.len() {
	// 					break;
	// 				}
	// 				current = chars[column];
	// 			}
	// 			// if word == "fun" {}
	// 			println!("[debug] tokens.push((Word, {:?}))", word);
	// 			tokens.push((TokenType::Word, Some(word)));
	// 			continue;
	// 		}
	// 		if DIGITS.contains(current) {
	// 			let mut number: String = String::new();
	// 			while DIGITS.contains(current) {
	// 				number.push(current);
	// 				column += 1;
	// 				if column >= chars.len() {
	// 					break;
	// 				}
	// 				current = chars[column];
	// 			}
	// 			// column -= 1;
	// 			// current = chars[column];
	// 			println!("[debug] tokens.push((Number, {}))", number);
	// 			tokens.push((TokenType::Number, Some(number)));
	// 			continue;
	// 		}
	// 		if BRACKETS.contains(current) {
	// 			tokens.push((TokenType::Bracket, Some(String::from(current))));
	// 			column += 1;
	// 			continue;
	// 		}
	// 		if current == '+' || current == '-' {
	// 			column += 1;
	// 			let op: char = current;
	// 			if column < chars.len() {
	// 				current = chars[column];
	// 				if current == op || current == '=' {
	// 					let mut op: String = String::from(op);
	// 					op.push(current);
	// 					println!("[debug] tokens.push((Operator, {:?}))", op);
	// 					tokens.push((TokenType::Operator, Some(op)));
	// 					column += 1;
	// 					continue;
	// 				}
	// 			}
	// 			println!("[debug] tokens.push((Operator, {:?}))", op);
	// 			tokens.push((TokenType::Operator, Some(String::from(op))));
	// 			continue;
	// 		}
	// 		println!("[debug] tokens = [");
	// 		for token in tokens {
	// 			match token.1 {
	// 				None => {
	// 					println!("    {}", format!("{:?}", token.0).to_lowercase())
	// 				},
	// 				Some(value) => println!("    {} {:?}", format!("{:?}", token.0).to_lowercase(), value)
	// 			};
	// 		}
	// 		println!("]");
	// 		eprintln!("{}: invalid character \"{}\"", red("SyntaxError"), current);
	// 		stack(file_path, row, column);
	// 		exit(1);
	// 	}
	// 	if expression_status != ExpressionStatus::Must {
	// 		println!();
	// 		println!("[debug] tokens.push((Eol, None))");
	// 		tokens.push((TokenType::Eol, None));
	// 	}
	// }

	println!();
	println!("[debug] tokens = {:?}", tokens);
	println!();

	// for (token_type, value) in tokens {
	// }
}

fn stack(path: String, row: usize, column: usize) {
	eprintln!("    at {}:{}:{}", path, row + 1, column + 1);
}

fn red(text: &str) -> String {
	String::from("\x1B[31m") + text + "\x1B[0m"
}

fn yellow(text: &str) -> String {
	String::from("\x1B[33m") + text + "\x1B[0m"
}

fn get_full_path(relative: String) -> String {
	let current_dir: PathBuf = match get_current_dir() {
		Err(error) => {
			eprintln!("{}: failed to get the current directory, {}", red("InternalError"), error);
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
			eprintln!("{}: file \"{}\" not found", red("LoadError"), full_path);
			exit(1);
		}
	}
	if !path.is_file() {
		eprintln!("{}: \"{}\" is not a file", red("LoadError"), full_path);
		exit(1);
	}
	match read_to_string(&path) {
		Err(error) => {
			eprintln!("{}: failed to read \"{}\", {}", red("LoadError"), full_path, error);
			exit(1);
		}
		Ok(file) => (resolve_path(path), file)
	}
}