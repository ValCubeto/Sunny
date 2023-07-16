use std::{fs, path::PathBuf, process::exit};
use crate::{errors::LoadError, about::EXTENSION};

pub fn read_file(path: &mut PathBuf) -> String {
	if !path.exists() {
		if
			path.extension().is_some()
			|| !path.with_extension(EXTENSION).exists()
		{
			LoadError!("file {path:?} not found");
		}
		path.set_extension(EXTENSION);
	}

	if !path.is_file() {
		LoadError!("{path:?} is not a file");
	}
	
	let bytes = match fs::read(&path) {
		Ok(bytes) => bytes,
		Err(err) => LoadError!("failed to read {path:?} ({err})")
	};

	let mut data = String::with_capacity(bytes.len() + 3);

	data.push('{');
	data.push('\n');
	match String::from_utf8(bytes) {
		Ok(code) => {
			let code = code.trim();
			if code.is_empty() {
				exit(0);
			}
			data.push_str(code);
		},
		Err(err) => LoadError!("the file {path:?} has invalid UTF-8 ({err})")
	};
	data.push('}');

	data
}