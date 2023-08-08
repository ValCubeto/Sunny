use aliases::Dict;

static GLOBAL: Vec<Dict> = vec![];

fn main() {
	let args = argv::parse_args();
	dbg!(&args);
}

mod aliases;
mod about;
mod macros;
mod argv;
mod values;
mod tests;