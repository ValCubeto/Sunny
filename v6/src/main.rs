use crate::{
	context::Context,
	dict::{Key, Dict},
	types::Value,
};

fn main() {
	let (exec_path, flags, main_path, args) = arg_parser::parse_args();

	let (code, main_path) = files::read_file(main_path);
	if code.is_empty() {
		// Warning!("empty file");
		return;
	}
	let ctx = &mut Context::new(main_path.clone(), &code);

	#[allow(unused)]
	let global_object = Dict::from([
		// convert into an instance of Process
		(Key::from("process"), Value::Dict(Dict::from([
			(Key::from("exec_path"), Value::String(exec_path)),
			(Key::from("flags"), Value::List(flags.iter().map(|v| Value::String(v.clone())).collect())),
			(Key::from("main_path"), Value::String(main_path)),
			(Key::from("args"), Value::List(args.iter().map(|v| Value::String(v.clone())).collect())),
		])))
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