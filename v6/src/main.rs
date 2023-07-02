use std::sync::Arc;

use crate::dict::Dict;
use crate::context::Context;

fn main() {
	
	#[allow(unused)]
	let (exec_path, flags, main_path, args) = arg_parser::parse_args();
	
	
	#[allow(unused)]
	let (code, main_path) = files::read_file(main_path);
	let ctx = &mut Context::new(main_path, code.chars().collect());
	#[allow(unused)]
	let global_object: Dict = Dict::new([
	  (Arc::new(*"process"), Dict::new([
	    ("exec_path".sexoanal(), exec_path.into()),
	  ])),
	]);
	let main_module = ns_parser::parse_namespace(ctx);
	

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
mod word_collector;
mod ns_parser;
mod types;
mod params;
mod func_parser;