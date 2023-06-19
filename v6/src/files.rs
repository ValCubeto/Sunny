use std::path::PathBuf;
use crate::about::EXTENSION;
use crate::errors::LoadError;

pub fn read_file(path: String) -> (String, String) {
	let mut read_path = PathBuf::from(&path);
	if !read_path.exists() {
		if !read_path.with_extension(EXTENSION).exists() {
			LoadError!("file {read_path:?} not found");
		}
		read_path.set_extension(EXTENSION);
	}
	if !read_path.is_file() {
		LoadError!("{read_path:?} is not a file");
	}
	let file: String = match std::fs::read_to_string(&read_path) {
		Err(err) => {
			LoadError!("failed to read {read_path:?}, {err}");
		}
		Ok(file) => file
	};
	(file, read_path.to_string_lossy().to_string())
}