mod parse_args;
mod about;
mod stdlib;
mod types;
mod colors;
mod paths;
mod files;
mod toml_stuff;

use types::{Any, Dict};

// auto-imported toml, crossterm
#[allow(unreachable_code)]

fn main() {
	#[allow(unused)]
	let (exec_path, flags, file, args): _ = parse_args::parse();

	let exec_path: String = paths::resolve(exec_path);
	println!("[debug] exec_path = {:?}", exec_path);

	let file: String = paths::resolve_filename(file);
	println!("[debug] main_file = {}", file);

	let workspace: _ = paths::dirname(file.clone());

	let cfg_path: _ = workspace.join("Sunny.toml");
	if cfg_path.exists() {
		println!("[debug] cfg file found: {:?}", cfg_path);
		let cfg_file: String = files::read(cfg_path.to_string_lossy().to_string());
	  let cfg: toml::map::Map<String, toml::Value> = toml_stuff::parse_toml(cfg_file);
		println!("[debug] cfg = {:?}", cfg);
	}
	return;
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