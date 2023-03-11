use std::process::ExitCode;
use std::env;
use std::path::Path;
use std::ffi::OsString;
use std::fs;
use std::collections::HashMap;

const EXTENSION: &str = ".sny";

fn main() -> ExitCode {
	let mut args: Vec<String> = env::args().collect();
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
	let mut path_arg: String = args.remove(0);

	dbg!(&executor_path);
	dbg!(&flags);
	dbg!(&path_arg);
	dbg!(&args);

	let (file_path, code): (OsString, String) = match try_read([&path_arg, &(path_arg.clone() + EXTENSION)]) {
		Ok(file) => file,
		Err(exit_code) => return exit_code
	};

	dbg!(&file_path);

	/*
	let mut tokens: Vec<Token> = Vec::new();
	tokens.push(Token {
		text: String::from("hello"),
		token_type: TokenType::Word
	});

	println!("{:#?}", tokens);
	*/
	return ExitCode::SUCCESS;
}

// returns the path read and the content of the file
fn try_read(paths: [&String; 2]) -> Result<(OsString, String), ExitCode> {
	for path in paths.clone() {
		if !Path::new(&path).exists() {
			continue;
		}
		if !Path::new(&path).is_file() {
			eprintln!("\"{}\" is not a file", path);
			return Err(ExitCode::FAILURE);
		}
		let file: String = match fs::read_to_string(&path) {
			Ok(data) => data,
			Err(error) => {
				eprintln!("Failed to read \"{}\": {}", path, error.kind());
				return Err(ExitCode::FAILURE);
			}
		};
		let real_path = match Path::new(&path).canonicalize() {
			Ok(new) => new.into_os_string(),
			Err(_) => {
				eprintln!("[cannot canonicalize the path argument]");
				OsString::from(path.clone())
			}
		};
		return Ok((real_path, file));
	}
	eprintln!("File \"{}\" not found", paths[0]);
	return Err(ExitCode::FAILURE);
}

#[derive(Debug)]
enum TokenType {
	None,
	Word
}

#[derive(Debug)]
struct Token {
	text: String,
	token_type: TokenType
}

#[derive(Debug)]
struct Flag {
	name: String,
	value: Option<String>
}