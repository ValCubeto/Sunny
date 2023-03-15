use std::io::ErrorKind;
use std::process::ExitCode;
use std::env::args as get_args;
use std::path::Path;
use std::fs::read_to_string;
// use std::collections::HashMap;

const EXTENSION: &str = ".sny";

fn main() -> ExitCode {
	let mut args: Vec<String> = get_args().collect();
	if args.len() < 2 {
		eprintln!("Missing arguments");
		return ExitCode::FAILURE;
	}
	let executor_path: String = args.remove(0);
	let mut flags: Vec<String> = Vec::new();

	for (i, arg) in args.clone().iter().enumerate() {
		if arg.len() < 1 || arg.chars().nth(0) != Some('-') /* || arg == "--" /* end of flags */ */ {
			break;
		}
		let flag: String = args.remove(i);
		flags.push(flag);
	}
	if args.len() < 1 {
		eprintln!("Missing path argument");
		return ExitCode::FAILURE;
	}
	let path_arg: String = args.remove(0);

	dbg!(&executor_path);
	dbg!(&flags);
	dbg!(&path_arg);
	dbg!(&args);

	let (file_path, code): (String, String) = match try_import(&path_arg) {
		Ok(file) => file,
		Err(error) => {
			if error.1 == ErrorKind::NotFound && !path_arg.ends_with(EXTENSION) {
				match try_import(&(path_arg.clone() + EXTENSION)) {
					Ok(file) => file,
					Err(error) => {
						eprintln!("{}", error.0);
						return ExitCode::FAILURE;
					}
				}
			} else {
				eprintln!("{}", error.0);
				return ExitCode::FAILURE;
			}
		}
	};

	dbg!(&file_path);
	dbg!(&code);

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

	let _operators = [
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
	let mut last_token: Option<Token> = None;
	let mut token_text: String = String::new();
	let mut token_type: TokenType = TokenType::Unknown;
	let mut context: Context = Context::Default;

	let mut in_comment = false;

	let mut quote: Option<char> = None;
	let mut escaping: bool = false;

	let mut line: usize = 1;
	let mut token_col: usize = 1;

	for (i, c) in code.char_indices() {
		if in_comment {
			if c == '\n' {
				println!("comment end");
				in_comment = false;
			}
			continue;
		}
		if context == Context::Default {
			println!("init at {}; {}:{}", i, line, token_col);
			if c == '\t' || c == ' ' {
				continue;
			}
			if c == '#' {
				println!("comment start");
				in_comment = true;
				continue;
			}
			if CLOSE_BRACKETS.contains(c) {
				eprintln!("SyntaxError: unexpected closing bracket '{}'\n    at {}:0:0", c, file_path);
			}
			token_type = match get_token_type(c.clone(), &file_path, line, token_col) {
				TokenType::Unknown => return ExitCode::FAILURE,
				other => other
			};

			if token_type == TokenType::Op {
				eprintln!("SyntaxError: unexpected operator '{}'\n    at {}:{}:{}", c, file_path, line, token_col);
				return ExitCode::FAILURE;
			}

			if token_type == TokenType::String { // ignore quotes
				println!("found string with quote {}", c);
				quote = Some(c);
				continue;
			}

			dbg!(&token_type);
			token_text.push(c);
			if c == '\n' {
				line += 1;
			}
			continue;
		}

		if token_type == TokenType::Unknown {
			println!("unknown token type found");
			if let Some(prev_token) = tokens.last() {
				println!("prev token type = {:?}", prev_token.token_type);
				if prev_token.token_type == TokenType::Word {
					// expect op | keyword
					if keywords.contains(&&prev_token.token_text[..]) {
						println!("{} is a kw", prev_token.token_text);
						if prev_token.token_text == "if" {
							context = Context::Expression;
							continue;
						}
					}
				}
			} else {
				eprintln!("InternalError: a token was not added to the token list when it ended\n    at {}:{}:{}", file_path, line, token_col);
				return ExitCode::FAILURE;
			}
		}

		if token_type == TokenType::Word {
			if WORD_CHARS.contains(c) {
				token_text.push(c);
				continue;
			}
			println!("end of word {}", token_text);
			tokens.push(Token { token_text, token_type });
			token_text = String::new();
			token_type = TokenType::Unknown;
		}

		if token_type == TokenType::String {
			if escaping {
				token_text.push(
					match c {
						'n' => '\n',
						'r' => '\r',
						't' => '\t',
						other => other
					}
				);
				escaping = false;
				continue;
			}
			if c == '\\' {
				escaping = true;
				continue;
			};
			if c == '\n' {
				eprintln!("SyntaxError: unexpected new line in string\n    at {}:{}:{}", file_path, line, token_col);
				return ExitCode::FAILURE;
			}
			if Some(c) == quote {
				println!("end of string with quote {:?}", quote);
				tokens.push(Token { token_text: token_text.clone(), token_type });
				token_type = TokenType::Unknown;
				quote = None;
				continue;
			}
			token_text.push(c);
		}

		if c == '\n' {
			if token_type == TokenType::Eol {
				line += 1;
				continue;
			}
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
	return ExitCode::SUCCESS;
}

// #[derive(Debug)]
// struct Flag {
// 	name: String,
// 	value: Option<String>
// }

// returns the full (if possible) path read and the content of the file
fn try_import(src: &String) -> Result<(String, String), (String, ErrorKind)> {
	let path: &Path = Path::new(src);
	if !path.exists() {
		return Err((format!("File \"{}\" not found", src), ErrorKind::NotFound));
	}
	if !path.is_file() {
		return Err((format!("\"{}\" is not a file", src), ErrorKind::InvalidInput));
	}
	let real_path: String = match path.canonicalize() {
		Ok(new) => new.into_os_string().to_string_lossy()[4..].to_string(), // remove the '\\?\' prefix
		Err(_) => {
			return Err((String::from("InternalError: failed to canonicalize the path argument"), ErrorKind::InvalidData));
		}
	};
	let file: String = match read_to_string(&src) {
		Ok(data) => data,
		Err(error) => {
			let error_kind: ErrorKind = error.kind();
			return Err((format!("Failed to read \"{}\": {}", real_path, error_kind), error_kind));
		}
	};
	return Ok((real_path, file));
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum TokenType {
	Unknown,
	Eol,
	Word,
	Number,
	String,
	Op,
	Bracket
}

#[derive(Debug)]
struct Token {
	token_text: String,
	token_type: TokenType
}

// const ALPHA_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
// const ALPHA_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
// const ALPHA      : &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const WORD_CHARS: &str = "abcdefghijklmnopqrstuvwxyz_ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS    : &str = "1234567890";
const OP_CHARS  : &str = "+-*/%=<>!";
const BRACKETS  : &str = "()[]{}";
const OPEN_BRACKETS: &str = "([{";
const CLOSE_BRACKETS: &str = ")]}";

fn get_token_type(c: char, file_path: &String, line: usize, token_col: usize) -> TokenType {
	if c == '\n' {
		return TokenType::Eol;
	}
	if c == '\'' || c == '"' {
		return TokenType::String;
	}
	if WORD_CHARS.contains(c) {
		return TokenType::Word;
	}
	if DIGITS.contains(c) {
		return TokenType::Number;
	}
	if OP_CHARS.contains(c) {
		return TokenType::Op;
	}
	if BRACKETS.contains(c) {
		return TokenType::Bracket;
	}
	eprintln!("SyntaxError: invalid character U+{:06x} '{}'\n    at {}:{}:{}", get_char_code(c), c, file_path, line, token_col);
	return TokenType::Unknown
}

#[derive(Debug, PartialEq)]
enum Context {
	Default,
	Expression,
	IfCondition,
	IfBody,
	FunctionName,
	FunctionParams,
	FunctionParamsEnd,
	FunctionReturnType,
	FunctionBody
}

fn get_char_code(c: char) -> u32 {
	return c as u32;
}

// why char can be converted into an u32 but u32 cannot be converted into a char?
fn char_from_char_code(code: u8) -> char {
	return code as char;
}
