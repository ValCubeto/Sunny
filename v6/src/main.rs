use crate::arg_parser::parse_args;

fn main() {
	#[allow(unused)]
	let (exec_path, flags, main_path, args) = parse_args();
	println!("exec_path = {:?}", exec_path);
	println!("flags = {:?}", flags);
	println!("main_path = {:?}", main_path);
	println!("args = {:?}", args);
}

mod arg_parser;
mod errors;
mod colors;
mod about;
