use std::env::{args_os, ArgsOs};
use std::ffi::OsString;
use std::process::ExitCode;
use std::{fs, path};
use std::collections::HashMap;

fn main() -> ExitCode {
	let args: ArgsOs = args_os();
	if args.len() == 0 {
		eprintln!("Missing arguments");
		return ExitCode::FAILURE;
	}
	let mut flags: Vec<OsString> = Vec::new();
	let mut i = 0;
	let mut last_flag_index = 0;

	for arg in args.into_iter() {
		if arg.len() < 1 || arg[0] != "-" {
			last_flag_index = i;
			break;
		}
		flags.push(arg.clone());
		i += 1;
	}

	let exec_args = args.section(last_flag_index, args.len() - 1);
	let file_relative_path = args[args.len() - 1];
	let file_path = realpath(file_relative_path);

	// check if exists
	// check is a file

	let file: fs::File = match fs::File::open(file_path) {
		Ok(file) => file,
		Err(error) => {
			eprintln!("Failed to read: {error}");
			return ExitCode::FAILURE;
		}
	};

	let mut tokens: Vec<Token> = Vec::new();
	tokens.push(Token { text: String::from("hello"), token_type: TokenType::Word });

	println!("{:#?}", tokens);
	return ExitCode::SUCCESS;
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
	name: OsString,
	value: Option<OsString> // OsString | None
}