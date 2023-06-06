use crate::arg_parser::parse_args;

fn main() {
	#[allow(unused)]
	let (exec_path, flags, main_path, args) = parse_args();

	// let file = read_file(main_path);
	// let global_object = Object::from([ ("process".into(), Object::from([ ("exec_path".into(), exec_path.into()) ])) ]);
	// let main_module = parse_module(file);
	// let arguments = Arguments::from([("0", args.into())]);
	// main_module.exec_function("main".into(), arguments, global_object);
}

mod arg_parser;
mod errors;
mod colors;
mod about;
