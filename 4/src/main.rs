// [[autofolding="${COMM}\s*#region**"]]

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
	for arg_os in args_os() {
		args.push(arg_os.to_string_lossy().to_string());
	}

	let executor_path: String = resolve_path(PathBuf::from(args.remove(0)));

	println!("[debug] executor_path = {:?}", executor_path);

	// #region flags
	let valid_flags = [
		// String::from("--eval"),
		String::from("--help"),
		String::from("--test"),
		String::from("--version")
	];

	let unique_flags = [
		String::from("--help"),
		String::from("--version")
	];

	let flag_map: HashMap<String, String> = HashMap::from([
		// (
		// 	String::from("-e"),
		// 	String::from("--eval")
		// ),
		(
			String::from("-h"),
			String::from("--help")
		),
		(
			String::from("-t"),
			String::from("--test")
		),
		(
			String::from("-v"),
			String::from("--version")
		)
	]);

	let mut flags: Vec<String> = vec![];

	for (i, mut arg) in args.clone().iter().enumerate() {
		let i: usize = i + 1;
		if arg == "-" {
			eprintln!("{}: invalid flag at position {}", red("ArgumentError"), i);
			exit(1);
		}
		if arg.len() < 2 || !arg.starts_with('-') {
			break;
		}
		if arg.len() == 2 {
			if !flag_map.contains_key(&args.remove(0)) {
				eprintln!("{}: unknown flag '{}'", red("ArgumentError"), arg);
				exit(1);
			}
			arg = &(flag_map[arg]);
		}
		if !valid_flags.contains(arg) {
			eprintln!("{}: unknown flag '{}'", red("ArgumentError"), arg);
			exit(1);
		}
		// if arg == "--eval" {
		// 	exit(0);
		// }
		if !unique_flags.contains(arg) {
			flags.push(arg.clone());
			continue;
		}
		if i != 1 {
			eprintln!("{}: unexpected flag '{}' at position {}", red("ArgumentError"), arg, i);
			exit(1);
		}
		if arg == "--help" {
			println!("{}", String::new()
				+ "\x1B[90m-h\x1B[0m | \x1B[90m--help   \x1B[0m    Shows this message\n"
				+ "\x1B[90m-v\x1B[0m | \x1B[90m--version\x1B[0m    Prints the current " + NAME + " version");
		} else if arg == "--version" {
			println!("{} {}", NAME, VERSION);
		} else {
			println!("{}: flag '{}' not implemented yet", red("ArgumentError"), arg);
		}
		exit(0);
	}

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