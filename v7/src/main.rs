use unicode_segmentation::{UnicodeSegmentation, Graphemes};
use crate::{args::ParsedArgs, context::Context, id::Id};

fn main() {
	let args: ParsedArgs = args::parse_args();
	println!("{args:#?}");

	let data: Graphemes = files::read_file(&args.main_path).graphemes(true);

	// let id = Id::from(args.main_path.file_stem());
	// let ctx = Context::new(id, data);
}

extern crate unicode_segmentation;

mod about;
mod errors;
mod args;
mod colors;
mod id;
mod files;
mod context;