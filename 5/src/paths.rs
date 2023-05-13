use crate::about::EXTENSION;
use crate::colors::error;
use std::path::PathBuf;
use std::process::exit;
use std::env::current_dir;

pub fn cwd() -> PathBuf {
	match current_dir() {
		Err(err) => {
			eprintln!("{}: failed to get the current directory, {}", error("LoadError"), err);
			exit(1);
		}
		Ok(dir) => dir
	}
}

pub fn resolve(path: String) -> String {
	cwd().join(path).to_string_lossy().to_string()
}

pub fn dirname(path: String) -> PathBuf {
	match PathBuf::from(path.clone()).parent() {
		None => {
			eprintln!("{}: failed to get the parent of {:?}", error("LoadError"), path);
			exit(1);
		}
		Some(parent) => {
			parent.to_path_buf() //.to_string_lossy().to_string()
		}
	}
}

pub fn resolve_filename(path: String) -> String {
	let path: String = resolve(path);
	let mut path_buf: PathBuf = PathBuf::from(path.clone());

	if path_buf.exists() {
		return path;
	}

	path_buf.set_extension(EXTENSION);
	if path_buf.exists() {
		return path_buf.to_string_lossy().to_string();
	}

	eprintln!("{}: file \"{}\" not found", error("LoadError"), path);
	exit(1);
}