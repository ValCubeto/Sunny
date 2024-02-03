use std::{ fs, path::PathBuf, process::exit };
use crate::{ load_error, about::EXTENSION };

pub fn read_namespace(path: &mut PathBuf) -> String {
	if !path.exists() {
		if
			path.extension().is_some()
			|| !path.with_extension(EXTENSION).exists()
		{
			load_error!("file {path:?} not found");
		}
		path.set_extension(EXTENSION);
	}

	if !path.is_file() {
		load_error!("{path:?} is not a file");
	}
	
	let bytes = match fs::read(&path) {
		Ok(bytes) => bytes,
		Err(err) => load_error!("failed to read {path:?} ({err})")
	};

	let mut data = String::from("{\n");

	match String::from_utf8(bytes) {
		Ok(code) => {
			let code = code.trim();
			if code.is_empty() {
				exit(0);
			}
			data.push_str(code);
		},
		Err(err) => load_error!("the file {path:?} has invalid UTF-8 ({err})")
	};

	data.push('}');

	data
}