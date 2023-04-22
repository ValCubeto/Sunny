mod parse_flags;
use std::collections::HashMap;
use std::env::{
	args_os,
	current_dir as get_current_dir
};
use std::process::exit;
use std::path::PathBuf;
use std::fs::read_to_string;

const NAME: &str = "Sunny";
const VERSION: &str = "1.0.0";

const EXTENSION: &str = "sny";

const SPACES: &str = "\u{20}\r\t";
const ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const WORD_CHARS: &str = "abcdefghijklmnopqrstuvwxyz_ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "1234567890";
const BRACKETS: &str = "(){}[]";

fn main() {
	let mut args: Vec<String> = vec![];

	// env::args() panics on invalid utf-8
	

	println!("[debug] flags = {:?}", flags);
	// #endregion flags

	if args.is_empty() {
		println!("{}: interactive mode not yet implemented", yellow("Warning"));
		eprintln!("{}: no arguments provided", red("ArgumentError"));
		exit(1);
	}

	let (file_path, file): (String, String) = read_file(args.remove(0));
	
	println!("[debug] file_path = {:?}", file_path);
	println!("[debug] args = {:?}", args);
}

fn stack(path: String, row: usize, column: usize) {
	eprintln!("    at {}:{}:{}", path, row + 1, column + 1);
}

fn red(text: &str) -> String {
	String::from("\x1B[31m") + text + "\x1B[0m"
}

fn yellow(text: &str) -> String {
	String::from("\x1B[33m") + text + "\x1B[0m"
}

fn get_full_path(relative: String) -> String {
	let current_dir: PathBuf = match get_current_dir() {
		Err(error) => {
			eprintln!("{}: failed to get the current directory, {}", red("InternalError"), error);
			exit(1);
		}
		Ok(dir) => dir
	};
	let path: PathBuf = current_dir.join(relative);
	path.to_string_lossy().to_string()
}

fn resolve_path(path: PathBuf) -> String {
	match path.canonicalize() {
		Err(error) => {
			eprintln!("LoadError: failed to resolve the path \"{}\", {}", path.to_string_lossy(), error);
			exit(1);
		}
		Ok(full_path) => {
			if std::env::consts::FAMILY == "windows" {
				// remove the '\\?\' prefix
				full_path.to_string_lossy()[4..].to_string()
			} else {
				full_path.to_string_lossy().to_string()
			}
		}
	}
} 

fn read_file(relative: String) -> (String, String) {
	let mut path: PathBuf = PathBuf::from(&relative);
	let full_path: String = get_full_path(relative);
	if !path.exists() {
		path.set_extension(EXTENSION);
		if !path.exists() {
			eprintln!("{}: file \"{}\" not found", red("LoadError"), full_path);
			exit(1);
		}
	}
	if !path.is_file() {
		eprintln!("{}: \"{}\" is not a file", red("LoadError"), full_path);
		exit(1);
	}
	match read_to_string(&path) {
		Err(error) => {
			eprintln!("{}: failed to read \"{}\", {}", red("LoadError"), full_path, error);
			exit(1);
		}
		Ok(file) => (resolve_path(path), file)
	}
}