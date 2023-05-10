mod parse_args;
mod about;
mod std;

fn main() {
	#[allow(unused)]
	let (executor_path, flags, file_path, args): _ = parse_args::parse();

	let size: crossterm::Result<(u16, u16)> = crossterm::terminal::size();

	println!("[test] terminal::size = {:?}", size);

	// #[allow(unused)]
	// let console: _ = console::init();
}