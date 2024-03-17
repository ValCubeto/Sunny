// pub mod math;

use std::process::ExitCode;
use std::env::{current_dir as get_current_dir, args as get_args};
use std::path::Path;
use std::env::consts::{FAMILY, ARCH, OS};
use std::io::ErrorKind;
use std::fs::read_to_string;
use std::collections::HashMap;

const NAME: &str = "Sunny";
const VERSION: &str = "1.0.0";
const EXTENSION: &str = ".sny"; // suffix

fn main() -> ExitCode {
	// if let Err(error) = get_current_dir() {
	// 	eprintln!("InternalError: failed to get the current working directory, {}", error);
	// 	return ExitCode::FAILURE;
	// }
	// TODO: catch invalid unicode
	let mut args: Vec<String> = get_args().collect();
	if args.len() < 2 {
		eprintln!("ArgumentError: missing arguments");
		return ExitCode::FAILURE;
	}

	let executor_arg: String = args.remove(0);
	let executor_path: String = match try_resolve_path(&executor_arg) {
		Err(err) => {
			eprintln!("InternalError: {}", err);
			return ExitCode::FAILURE;
		},
		Ok(path) => path,
	};
	let mut flags: Vec<String> = Vec::new();

	let first_flags = [
		"-h", "--help",
		"-v", "--version"
	];

	for (i, arg) in args.clone().iter().enumerate() {
		if arg.len() < 2 || !arg.starts_with('-') {
			break;
		}
		if first_flags.contains(&&arg[..]) {
			if i != 0 {
				eprintln!("ArgumentError: unexpected flag \x1B[90m{}\x1B[0m at position {}", arg, i + 1);
				return ExitCode::FAILURE;
			}
			if arg == "--help" || arg == "-h" {
				let help_message: String = String::new()
					+ "\x1B[90m-h\x1B[0m | \x1B[90m--help   \x1B[0m    Shows this message\n"
					+ "\x1B[90m-v\x1B[0m | \x1B[90m--version\x1B[0m    Prints the current " + NAME + " version\n";
				print!("{}", help_message);
			} else if arg == "--version" || arg == "-v" {
				println!("{} {}", NAME, VERSION);
			}
			return ExitCode::SUCCESS;
		}
		let flag: String = args.remove(0);
		flags.push(flag);
	}

	if args.is_empty() {
		eprintln!("ArgumentError: missing path argument");
		return ExitCode::FAILURE;
	}
	let path_arg: String = args.remove(0);

	let (used_path, code): (String, String) = match read_file(&path_arg) {
		Err(error) => {
			if error.1 != ErrorKind::NotFound || path_arg.ends_with(EXTENSION) {
				eprintln!("LoadError: {}", error.0);
				return ExitCode::FAILURE;
			}
			let real_path: String = path_arg.clone() + EXTENSION;
			match read_file(&real_path) {
				Err(suberr) => {
					eprintln!("LoadError: {}", if suberr.1 == ErrorKind::NotFound { error.0 } else { suberr.0 });
					return ExitCode::FAILURE;
				},
				Ok(file) => (real_path, file)
			}
		},
		Ok(file) => (path_arg.clone(), file)
	};

	let file_path: String = match try_resolve_path(&used_path) {
		Err(err) => {
			eprintln!("InternalError: {}", err);
			return ExitCode::FAILURE;
		},
		Ok(path) => path
	};

	dbg!(&executor_arg, &path_arg, &executor_path, &flags, &file_path, &args);

	
	let keywords = [
		"var",
		"const",
		"fun",
		"class",
		"typedef",

		"if",
		"else",
		"match",
		
		"for",
		"in",
		"of",

		"and",
		"or",

		"import",
		"from",
		"as",

		"extends",
	];

	
	let operators = [
		"+",
		"++",
		"-",
		"--",
		"*",
		"**",
		"/",
		"%",
	];

	let mut tokens: Vec<Token> = Vec::new();
	let mut prev_token: Token = Token(TokenType::Space, String::new(), 1, 1);
	let mut token_text: String = String::new();
	let mut token_type: TokenType = TokenType::Space; // placeholder
	let mut line: usize = 1;
	let mut token_col: usize = 1;

	// space, eol
	let mut space_count: usize = 0;

	// comments
	let mut in_comment: bool = false;

	// strings
	let mut quote: Option<char> = None;
	let mut escaping: bool = false;
	
	for (i, c) in code.char_indices() {
		if in_comment {
			if c == '\n' {
				println!("comment: #{}", token_text);
				token_text = String::new();
				in_comment = false;
				line += 1;
			} else {
				token_text.push(c);
			}
			continue;
		}
		if token_text.is_empty() {
			if c == '#' {
				in_comment = true;
				continue;
			}
			if c == '\'' || c == '"' {
				quote = Some(c);
				continue;
			}
			// token_col = 
			token_text.push(c);
			token_type = match get_token_type(c, &file_path, line, token_col) {
				Err(error) => {
					eprintln!("SyntaxError: {}", error);
					return ExitCode::FAILURE;
				},
				Ok(t) => t
			};
			continue;
		}
		if prev_token.0 != token_type {
			if prev_token.0 == TokenType::Space || prev_token.0 == TokenType::Eol {
				space_count = 0;
			}
			let token: Token = Token(token_type, token_text.clone(), line, token_col);
			tokens.push(token.clone());
			prev_token = token;
			token_text = String::new();
			token_col = i;
			continue;
		}
		if token_type == TokenType::Space {
			space_count += 1;
			continue;
		}
		if token_type == TokenType::Eol {
			space_count += 1;
			line += 1;
			continue;
		}
		if token_type == TokenType::Word {
			token_text.push(c);
			continue;
		}
		if token_type == TokenType::Number {
			if escaping {
				if !DIGITS.contains(c) {
					eprintln!("SyntaxError: unexpected character {}\n    at {}:{}:{}", get_unicode_repr(c), file_path, line, token_col);
					return ExitCode::FAILURE;
				}
				continue;
			}
			if c == '\'' {
				escaping = true;
				continue;
			} else {
				escaping = false;
			}
			token_text.push(c);
		}
	}

	dbg!(&tokens);

	if token_type == TokenType::String {
		eprintln!("SyntaxError: unexpected end of file, unclosed string");
		return ExitCode::FAILURE;
	}
	/*
		code = "x = 'Hola'"
		tokens = [
			{ token_text: "x",    token_type: Word   }
			{ token_text: "=",    token_type: Op     }
			{ token_text: "Hola", token_type: String }
		]
	*/
	ExitCode::SUCCESS
}

/*
#[allow(unused, non_camel_case_types)]
enum Range {
	num(String, String),
	char(char, char)
}

fn get_real_path(relative_path: &String) -> Result<PathBuf, ErrorKind> {
	match get_current_dir() {
		Err(e) => Err(e.kind()),
		Ok(cwd) => Ok(cwd.join(relative_path)),
	}
}
*/

/** if fails returns the relative path back */
fn try_resolve_path(relative_path: &String) -> Result<String, String> {
	let path: &Path = Path::new(relative_path);
	if path.is_absolute() {
		return Ok(relative_path.clone());
	}
	match path.canonicalize() {
		Err(_) => match get_current_dir() {
			Err(err) => {
				Err(format!("failed to get the current directory, {}", err))
			},
			Ok(dir) => Ok(dir.join(relative_path).to_string_lossy().to_string())
		},
		Ok(real_path) => {
			if FAMILY == "windows" {
				// remove the "\\?\" prefix
				Ok(real_path.to_string_lossy()[4..].to_string())
			} else {
				Ok(real_path.to_string_lossy().to_string())
			}
		}
	}
}

/** Returns the used path and the content of the file */
fn read_file(src: &String) -> Result<String, (String, ErrorKind)> {
	let full_src: String = match try_resolve_path(src) {
		Err(err) => {
			return Err((err, ErrorKind::AddrNotAvailable))
		},
		Ok(path) => path
	};
	let path: &Path = Path::new(&full_src);
	if !path.exists() {
		return Err((format!("file \"{}\" not found", full_src), ErrorKind::NotFound));
	}
	if !path.is_file() {
		return Err((format!("\"{}\" is not a file", full_src), ErrorKind::InvalidInput));
	}
	let file: String = match read_to_string(src) {
		Err(error) => {
			let error_kind: ErrorKind = error.kind();
			return Err((format!("failed to read \"{}\", {}", full_src, error_kind), error_kind));
		},
		Ok(data) => data
	};
	Ok(file)
}


#[derive(Debug, PartialEq, Clone, Copy)]
enum TokenType {
	Eol, // \n
	Space, // \u0020, \t, \r
	Word, // [a-z_A-Z]
	Number, // [0-9]
	String, // '**' | "**"
	Op, // + - * /
	Bracket // () {} []
}

#[derive(Debug, Clone)]
struct Token(TokenType, String, usize, usize);

// const ALPHA_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
// const ALPHA_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
// const ALPHA      : &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

const WORD_CHARS: &str = "abcdefghijklmnopqrstuvwxyz_ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "1234567890";
const OP_CHARS: &str = "+-*/%=<>!";
const BRACKETS: &str = "()[]{}";
const OPEN_BRACKETS: &str = "([{";
const CLOSE_BRACKETS: &str = ")]}";


fn get_token_type(c: char, file_path: &String, line: usize, token_col: usize) -> Result<TokenType, String> {
	if c == '\n' {
		return Ok(TokenType::Eol);
	}
	if c == ' ' || c == '\t' || c == '\r' {
		return Ok(TokenType::Space);
	}
	if c == '\'' || c == '"' {
		return Ok(TokenType::String);
	}
	if WORD_CHARS.contains(c) {
		return Ok(TokenType::Word);
	}
	if DIGITS.contains(c) {
		return Ok(TokenType::Number);
	}
	if OP_CHARS.contains(c) {
		return Ok(TokenType::Op);
	}
	if BRACKETS.contains(c) {
		return Ok(TokenType::Bracket);
	}
	Err(format!("invalid character {} '{}'\n    at {}:{}:{}", get_unicode_repr(c), c, file_path, line, token_col))
}

fn get_char_code(c: char) -> u32 {
	c as u32
}

fn get_unicode_repr(c: char) -> String {
	format!("U+{:06x}", get_char_code(c))
}


// why char can be converted into an u32 but u32 cannot be converted into a char?
fn char_from_char_code(code: u8) -> char {
	code as char
}
