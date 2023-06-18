use crate::context::Context;

fn main() {
	
	#[allow(unused)]
	let (exec_path, flags, main_path, args) = arg_parser::parse_args();
	
	// let global_object = Object::from([
	//   ("process".into(), Object::from([
	//     ("exec_path".into(), exec_path.into())
	//   ]))
	// ]);
	
	#[allow(unused)]
	let (file, main_path) = files::read_file(main_path);
	dbg!(&main_path, &file);
	
	let ctx = &mut Context::new(main_path, file.chars().collect());
	#[allow(unused)]
	let main_module = ns_parser::parse_namespace(ctx);

	// let arguments = Arguments::from([("0", args.into())]);

	// main_module.exec_function("main".into(), arguments, global_object);
}

mod colors;
mod errors;
mod arg_parser;
mod about;
mod files;
mod context;
mod ns_parser;
mod types;
mod params;
mod func_parser;