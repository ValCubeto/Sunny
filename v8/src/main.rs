use crate::aliases::Id;

fn main() {
	let mut args = argv::parse_args();
	println!("args = {args:#?}");

	let data = files::read_namespace(&mut args.main_path);
	println!("data = {data:?}");

	println!();

	let file_id = Id::from(args.main_path
		.file_name()
		.unwrap() // guaranteed to be a file
		.to_string_lossy()
		.to_string());
	let path_id = Id::from(args.main_path
		.to_string_lossy()
		.to_string());

	let mut ctx = &mut Context::new(path_id, data);

	let main_namespace = parse_namespace(ctx, file_id);
	// make_global();
}

mod aliases;
mod about;
mod colors;
mod macros;
mod argv;
mod files;
mod values;
mod arguments;
mod functions;
mod expressions;
mod tests;