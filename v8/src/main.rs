fn main() {
	let args = argv::parse_args();
	dbg!(&args);

	// make_global();
}

mod aliases;
mod about;
mod colors;
mod macros;
mod argv;
mod values;
mod tests;