mod parse_args;
mod about;
mod stdlib;
mod types;
mod colors;
mod paths;
mod files;
mod toml_stuff;
mod errors;
mod namespaces;
mod words;
mod functions;

use crate::types::{Any, Dict};
use crate::errors::{debug, Warning};
use crate::files::read;
use crate::namespaces::parse_namespace;
use std::collections::HashMap;

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

	#[allow(unused)]
	let mut global: Dict = Dict::from([
		("process", Any::Dict(&Dict::from([
			("exec_path", Any::String(exec_path)),
			("flags", Any::List(&flags.iter().map(|v: &String| Any::String(v.clone())).collect())),
			("file_path", Any::String(file_path.clone())),
			("args", Any::List(&args.iter().map(|v: &String| Any::String(v.clone())).collect())),
			// ("get_title")
			// ("set_title", |title: String|)
		]))),
		("none", Any::None),
		("infinity", Any::Infinity),
	]);

	let chars: Vec<char> = read(file_path).chars().collect();
	debug!("code = {:?}", chars.iter().collect::<String>());

	println!();

	#[allow(unused)]
	let mut i: usize = 0;

	let main: HashMap<String, Any> = parse_namespace(&chars, &mut i);

	if main.contains_key("exports") {
		Warning!("exported values found in the main file");
	}

	#[allow(unused)]
	pub fn collect_comment(chars: &[char], i: &mut usize) {
		while *i < chars.len() {
			let chr: char = chars[*i];
			// debug!("chr = {:?}", chr);
			if chr == '\n' {
				break;
			}
			*i += 1;
		}
	}

	#[allow(unused)]
	fn collect_word(chars: &[char], i: &mut usize) -> String {
		let mut word: String = String::new();
		while *i < chars.len() {
			let chr: char = chars[*i];
			match chr {
				'a'..='z' | '_' | 'A'..='Z' => {
					word.push(chr);
				}
				_ => {
					break;
				}
			}
			*i += 1;
		}
		word
	}
}
