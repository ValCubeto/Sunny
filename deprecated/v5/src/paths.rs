use crate::about::EXTENSION;
use crate::errors::LoadError;
use std::path::PathBuf;
use std::env::current_dir;

pub fn cwd() -> PathBuf {
	match current_dir() {
		Err(err) => {
			LoadError!("failed to get the current directory, {err}");
		}
		Ok(dir) => dir
	}
}

pub fn resolve(path: String) -> String {
	// no hay errores porque join elimina todo lo anterior despues de una raiz
	let path = match cwd().canonicalize() {
		Err(_) => { LoadError!("que"); },
		Ok(p) => p
	}.join(path);
	if cfg!(windows) {
		path.to_string_lossy()[4..].to_string()
	} else {
		path.to_string_lossy().to_string()
	}
}

pub fn dirname(path: String) -> PathBuf {
	match PathBuf::from(path.clone()).parent() {
		None => {
			LoadError!("failed to get the parent of {:?}", path);
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
		if !path_buf.is_file() {
			LoadError!("\"{path}\" is not a file");
		}
		return path
	}
	
	path_buf.set_extension(EXTENSION);
	if path_buf.exists() {
		if !path_buf.is_file() {
			LoadError!("\"{path}\" is not a file");
		}
		return path_buf.to_string_lossy().to_string();
	}
	LoadError!("file \"{path}\" not found");
}