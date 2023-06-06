use std::{env::args_os, process::exit};
use crate::errors::{InternalError, ArgumentError};

pub fn parse_args() -> (String, Vec<String>, String, Vec<String>) {
	let mut args_os = args_os().into_iter();

	let exec_path: String = match args_os.next() {
		None => {
			InternalError!("args_os is empty");
		}
		Some(os_string) => os_string.to_string_lossy().to_string()
	};

	let mut flags: Vec<String> = Vec::new();

	let mut main_path = String::new();

	for arg in &mut args_os {
		let arg = arg.to_string_lossy().to_string();
		// println!("arg = {}", arg);
		if !arg.starts_with('-') {
			main_path = arg;
			break;
		};
		match arg.as_str() {
			"-v" | "--version" => {
				if args_os.len() != 0 {
					println!("Warning: unused {} extra arguments", args_os.len())
				}
				println!("Sunny 1.0.0");
				exit(0);
			}
			"--test" => {
				flags.push(arg);
			}
			_ => {
				ArgumentError!("invalid flag {:?}", arg);
			}
		}
	}

	if main_path.is_empty() {
		ArgumentError!("missing file path");
	}

	let mut args: Vec<String> = Vec::new();

	for arg in &mut args_os {
		args.push(arg.to_string_lossy().to_string())
	}

	(exec_path, flags, main_path, args)
}