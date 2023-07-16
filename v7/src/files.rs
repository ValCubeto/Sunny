use std::{fs, path::PathBuf};
use crate::errors::LoadError;

pub fn read_file(path: &PathBuf) -> String {
	if !path.is_file() {
		LoadError!("{path:?} is not a file");
	}
	
	let bytes = match fs::read(path) {
		Ok(bytes) => bytes,
		Err(err) => LoadError!("failed to read {path:?}; {err}")
	};

	match String::from_utf8(bytes) {
		Ok(data) => data,
		Err(err) => LoadError!("the file {path:?} has invalid UTF-8, {err}")
	}
}