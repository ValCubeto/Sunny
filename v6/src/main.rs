use crate::arg_parser::parse_args;

fn main() {
	#[allow(unused)]
	let (exec_path, flags, main_path, args) = parse_args();
}

mod arg_parser;
mod errors;
mod colors;
