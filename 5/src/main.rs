mod parse_args;
mod about;
mod stdlib;
mod types;
mod colors;
mod paths;
mod files;
mod toml_stuff;
mod errors;

use types::{Any, Dict};

use crate::{errors::{debug, InternalError}, files::read};

// auto-imported toml, crossterm

fn main() {
	#[allow(unused)]
	let (exec_path, flags, file_path, args): _ = parse_args::parse();

	let exec_path: String = paths::resolve(exec_path);
	debug!("exec_path = {:?}", exec_path);

	let file_path: String = paths::resolve_filename(if file_path == "." { String::from("main.sny") } else { file_path });
	debug!("file_path = {:?}", file_path);

	let workspace: _ = paths::dirname(file_path.clone());

	let cfg_path: _ = workspace.join("Sunny.toml");
	if cfg_path.exists() {
		debug!("cfg file found: {:?}", cfg_path);
		let cfg_file: String = files::read(cfg_path.to_string_lossy().to_string());
	  let cfg: toml::map::Map<String, toml::Value> = toml_stuff::parse_toml(cfg_file);
		debug!("cfg = {:?}", cfg);
	}
	// let std = std::init();

	let mut global: Dict = Dict::from([
		("process".to_string(), Any::Dict(Dict::from([
			("exec_path".to_string(), Any::String(exec_path)),
			("flags".to_string(), Any::List(flags.iter().map(|v: &String| Any::String(v.clone())).collect())),
			("file_path".to_string(), Any::String(file_path.clone())),
			("args".to_string(), Any::List(args.iter().map(|v: &String| Any::String(v.clone())).collect()))
		]))),
		("none".to_string(), Any::None),
		("infinity".to_string(), Any::Infinity),
	]);

	let code: String = read(file_path.clone());
	debug!("code = {:?}", code);
		
	let mut i: usize = 0;
	while i < code.len() {
		let curr: char = match code.chars().nth(i) {
			None => {
				InternalError!("\"{}\" out of bounds", i);
			}
			Some(c) => c
		};
		debug!("code[{}] = {:?}", i, curr);
		i += 1;
	}
}