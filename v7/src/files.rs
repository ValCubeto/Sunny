use {
	std::{
		fs::read as fs_read,
		path::PathBuf,
		process::exit
	},
	crate::{
		about::EXTENSION,
		load_error
	}
};

pub fn read_file(path: &mut PathBuf) -> String {
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
	
	let bytes = match fs_read(&path) {
		Ok(bytes) => bytes,
		Err(err) => load_error!("failed to read {path:?} ({err})")
	};

	let mut data = "{\n".to_owned();

	match String::from_utf8(bytes) {
		Ok(code) => {
			let code = code.trim();
			if code.is_empty() {
				exit(0);
			}
			data.push_str(code);
		},
		Err(err) => load_error!("the file {path:?} must be valid UTF-8 ({err})")
	};
	data.push('}');

	data
}