use ::std::process::exit;

mod parse_args;
mod about;
mod std;
mod types;

use types::{Dict, Any};

fn main() {
	#[allow(unused)]
	let (exec_path, flags, file, args): _ = parse_args::parse();

	// let std = std::init();

	use toml::Table;

	let value: Table = match "foo = \n'bar'".parse::<Table>() {
		Ok(data) => {
			data
		},
		Err(err) => {
			eprintln!("erro {}", err);
			exit(1);
		}
	};

	dbg!(&value);

	#[allow(unused)]
	let mut global: Dict = Dict::from([
		("process".to_string(), Any::Dict(Dict::from([
			("exec_path".to_string(), Any::String(exec_path)),
			("flags".to_string(), Any::List(flags.iter().map(|v: &String| Any::String(v.clone())).collect())),
			("file".to_string(), Any::String(file)),
			("args".to_string(), Any::List(args.iter().map(|v: &String| Any::String(v.clone())).collect()))
		]))),
		("none".to_string(), Any::None),
		("infinity".to_string(), Any::Infinity),
	]);

	println!("global = {:?}", global);
}