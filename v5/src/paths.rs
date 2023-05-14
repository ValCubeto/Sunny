use crate::about::EXTENSION;
use crate::errors::LoadError;
use std::path::PathBuf;
use std::env::current_dir;

pub fn cwd() -> PathBuf {
	match current_dir() {
		Err(err) => {
			LoadError!("failed to get the current directory, {}", err);
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
			LoadError!("\"{}\" is not a file", path);
		}
		return /* match */ path/* _buf.canonicalize() {
			Err(err) => {
				LoadError!("the path \"{}\" exists, but something went wrong resolving it. {}", path, err);
			}
			Ok(path) => path.to_string_lossy().to_string()
		}; */
	}
	
	path_buf.set_extension(EXTENSION);
	if path_buf.exists() {
		if !path_buf.is_file() {
			LoadError!("\"{}\" is not a file", path);
		}
		return /* match */ path_buf.to_string_lossy().to_string()/* _buf.canonicalize() {
			Err(err) => {
				LoadError!("the path \"{}\" exists, but something went wrong resolving it. {}", path, err);
			}
			Ok(path) => path.to_string_lossy().to_string()
		}; */
	}
	LoadError!("file \"{}\" not found", path);
}