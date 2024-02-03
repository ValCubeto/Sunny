use crate::{
	context::Context,
	dict::Dict
};

fn main() {
	let (exec_path, flags, main_path, args) = arg_parser::parse_args();

	let (code, main_path, id) = files::read_file(main_path);
	let ctx = &mut Context::new(id, &code);

	// if code.len() < id.len() + 4 {
	// 	return;
	// }

	#[allow(unused)]
	let global_object = Dict::from([
		// convert into an instance of Process
		("process", Dict::from([
			("exec_path", exec_path.into()),
			("flags",     flags.into()),
			("main_path", main_path.into()),
			("args",      args.into()),
		]).into())
	]);

	let main_module = ns_parser::parse_namespace(ctx);
	
	dbg!(&main_module);

	// let arguments = Arguments::from([(0, args.into())]);
	// main_module.exec_function("main".into(), arguments, global_object);
}

mod colors;
mod errors;
mod arg_parser;
mod about;
mod dict;
mod files;
mod context;
mod ns_parser;
mod types;
mod params;
mod func_parser;
mod structs;
mod expressions;