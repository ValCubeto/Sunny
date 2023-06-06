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

	#[allow(unused)]
	let main_module = mod_parser::parse_module(file, main_path.clone());

	// let arguments = Arguments::from([("0", args.into())]);

	// main_module.exec_function("main".into(), arguments, global_object);
}

mod colors;
mod errors;
mod arg_parser;
mod about;
mod files;
mod context;
mod mod_parser;
mod func_parser;