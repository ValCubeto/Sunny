use args::ParsedArgs;

fn main() {
	let args: ParsedArgs = args::parse_args();
	println!("{:?}", args);
	// let file = files::read_bytes();
}

mod about;
mod errors;
mod args;
mod colors;
mod id;
mod files;