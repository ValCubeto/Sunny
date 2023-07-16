use crate::{
	args::{ParsedArgs, parse_args},
	id::Id,
	context::Context, namespaces::parse_namespace, errors::{ReferenceError, TypeError}, values::Value,
};

fn main() {
	let mut args: ParsedArgs = parse_args();	
	let data = files::read_file(&mut args.main_path);

	println!("args = {args:#?}");
	println!("data = {:?}", data);
	println!();

	let id = Id::from(args.main_path
		.file_name()
		.unwrap()
		.to_string_lossy()
		.to_string()
		.as_str());
	let ctx = &mut Context::new(id.clone(), &data);
	let main = parse_namespace(ctx, id);
	let entrypoint = match main.get(&Id::from("main")) {
		Some(value) => value,
		None => ReferenceError!(ctx, "")
	};
	if let Value::Function(_function) = entrypoint {
		// function.call(arguments);
	} else {
		TypeError!(ctx, r#""main" must be of type "function", got {:?}"#, entrypoint.typename());
	}
}

extern crate unicode_segmentation;

mod about;
mod errors;
mod args;
mod colors;
mod id;
mod files;
mod context;
mod namespaces;
mod values;
mod functions;