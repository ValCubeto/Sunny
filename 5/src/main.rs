
mod parse_args;
mod about;
mod stdlib;
mod types;
mod colors;

#[allow(unused)]
use std::process::exit;
#[allow(unused)]
use types::{Any, Dict};

// auto-imported toml, crossterm

fn main() {
	#[allow(unused)]
	let (exec_path, flags, file, args): _ = parse_args::parse();

	// let exec_path = resolve(exec_path);
	// let file = resolve_filename(file);
	// let workspace = path::dirname(file);
	// let cfg_path = path::join(workspace, "Sunny.toml");
	// if exists(config_path) {
	//   let cfg = read_to_string(cfg_path).parse::<toml::Table>();
	// }

	// let std = std::init();

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