use crate::{
	argv::{ ParsedArgs, parse_args },
	files::read_file,
	aliases::{ Id, Arguments },
	context::Context,
	namespaces::parse_namespace,
	values::Value,
	global::make_global,
};

fn main() {
	let mut args: ParsedArgs = parse_args();	
	let data = read_file(&mut args.main_path);

	println!("args = {args:#?}");
	println!("data = {:?}", data);
	println!();

	let file_id = Id::from(args.main_path
		.file_name()
		.unwrap() // guaranteed to be a file
		.to_string_lossy()
		.to_string());
	let path_id = Id::from(args.main_path
		.to_string_lossy()
		.to_string());

	let mut ctx = Context::new(path_id, &data);
	ctx.stack.push(make_global());
	let main = parse_namespace(&mut ctx, file_id);
	
	let entrypoint = match main.data.get(&Id::from("main")).cloned() {
		Some(value) => value,
		None => reference_error!("main function not found"; ctx)
	};
	ctx.stack.push(main.data);
	dbg!(&ctx.stack);

	if let Value::Function(function) = entrypoint {
		// if function.unwrap_defined().is_async {
		// 	type_error!("the main function cannot be async");
		// }
		let arguments = Arguments::new();
		match function.call(arguments) {
			Ok(_) => (),
			Err(e) => panic!("{e:?}")
		};
	} else {
		type_error!("missing main function"; ctx);
	}
}

mod about;
mod macros;
mod argv;
mod colors;
mod aliases;
mod files;
mod context;
mod namespaces;
mod values;
mod functions;
mod statments;
mod arguments;
mod numbers;
mod expressions;
mod eval;
mod stack;
mod structs;
mod instances;
mod global;

#[cfg(test)]
mod tests;