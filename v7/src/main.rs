use crate::{
	argv::{ ParsedArgs, parse_args },
	files::read_file,
	id::Id,
	context::Context,
	namespaces::parse_namespace,
	values::Value,
	arguments::Arguments,
};

fn main() {
	let mut args: ParsedArgs = parse_args();	
	let data = read_file(&mut args.main_path);

	println!("args = {args:#?}");
	println!("data = {:?}", data);
	println!();

	let file_id = Id::from(args.main_path
		.file_name()
		.unwrap() // `read_file` already proved that the path is a file
		.to_string_lossy()
		.to_string());
	let path_id = Id::from(args.main_path
		.to_string_lossy()
		.to_string());

	let mut ctx = Context::new(path_id, &data);
	let global = parse_namespace(&mut ctx, file_id);
	
	let entrypoint = match global.data.get(&Id::from("main")).cloned() {
		Some(value) => value,
		None => ReferenceError!(ctx, "main function not found")
	};
	ctx.stack.vec.push(global);
	dbg!(&ctx.stack);

	if let Value::Function(function) = entrypoint {
		if function.is_async {
			TypeError!("the main function cannot be async");
		}
		let arguments = Arguments::new();
		ctx.call_fun(function, arguments);
	} else {
		TypeError!(ctx, "main function not found, found {:?}", entrypoint.typename());
	}
}

mod about;
mod macros;
mod argv;
mod colors;
mod id;
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

#[cfg(test)]
mod tests;