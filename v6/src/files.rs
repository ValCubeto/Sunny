use std::{
	io::Read,
	path::PathBuf,
	fs::File
};
use crate::{
	about::EXTENSION,
	errors::{LoadError, InternalError}
};

pub fn read_file(path: String) -> (String, String, String) {
	let mut read_path = PathBuf::from(&path);
	if !read_path.exists() {
		if
			read_path.extension().is_some()
			|| !read_path.with_extension(EXTENSION).exists()
		{
			LoadError!("file \"{}\" not found", read_path.display());
		}
		read_path.set_extension(EXTENSION);
	}
	if !read_path.is_file() {
		LoadError!("{read_path:?} is not a file");
	}
	let mut stem: String = match read_path.file_stem() {
		None => {
			InternalError!("failed to get the file stem in {read_path:?}");
		}
		Some(stem) => {
			stem.to_string_lossy().to_string()
		}
	};
	let mut code = stem.clone();
	code.push('{');
	code.push('\n');
	
	let mut file = match File::open(&read_path) {
		Err(err) => {
			LoadError!("failed to open {read_path:?}. {err}");
		}
		Ok(data) => data
	};
	
	let mut buffer = Vec::new();
	if let Err(err) = file.read_to_end(&mut buffer) {
		LoadError!("failed to read {read_path:?}. {err}");
	}

	let content: String = match String::from_utf8(buffer) {
		Err(err) => {
			LoadError!("the file {read_path:?} has invalid data. {err}");
		}
		Ok(data) => data
	};
	code.push_str(content.as_str());

	code.push('}');
	(code, read_path.to_string_lossy().to_string(), stem)
}