use std::env::{args_os, ArgsOs};
use std::ffi::OsString;

fn main() {
	let args: ArgsOs = args_os();
	let mut flags: Vec<OsString> = Vec::new();

	for arg in args.into_iter() {
		if arg.is_empty() || arg[0] != "-" {
			break;
		}
		flags.push(arg.clone());
	}

	let mut tokens: Vec<Token> = Vec::new();
	tokens.push(Token { text: String::from("hello"), token_type: Type::Word });

	println!("{:#?}", tokens);
}

#[derive(Debug)]
struct Token {
	text: String,
	token_type: Type
}

#[derive(Debug)]
enum Type {
	None,
	Word
}