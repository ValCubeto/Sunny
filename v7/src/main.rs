use crate::{
	args::{ParsedArgs, parse_args},
	id::Id,
	context::Context, namespaces::parse_namespace,
};

fn main() {
	let mut args: ParsedArgs = parse_args();	
	let data = files::read_file(&mut args.main_path);

	println!("args = {args:#?}");
	println!("data = {:?}", data);
	println!();

	let id = Id::from(args.main_path
		.file_stem()
		.unwrap()
		.to_string_lossy()
		.to_string()
		.as_str());
	let ctx = &mut Context::new(id.clone(), data);
	let main = parse_namespace(ctx, id);
	// match main.get("main") {
	// 	Some(value) => match value {
	// 		Value::Function(function) => function
	// 	}
	// }
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