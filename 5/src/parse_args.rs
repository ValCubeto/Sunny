use std::collections::HashMap;
use std::env::args_os;
use std::process::exit;
use crate::about::{NAME, VERSION};

fn error(text: &str) -> String {
	format!("\u{1b}[31m\u{1b}[1m{}\u{1b}[22m\u{1b}[39m", text)
}

pub fn new() -> (String, Vec<String>, String, Vec<String>) {
	let mut args: Vec<String> = vec![];
	for arg_os in args_os() {
		// env::args() panics on invalid unicode
		args.push(arg_os.to_string_lossy().to_string());
	}

	let executor: String = args.remove(0);

	println!("[debug] executor_path = {:?}", executor);

	if args.is_empty() {
		eprintln!("{}: REPL not implemented", error("ArgumentError"));
		exit(1);
	}

	// #region flags
	let flag_map: HashMap<&str, &str> = HashMap::from([
		// ("-e", "--eval"),
		("-h", "--help"),
		("-t", "--test"),
		("-v", "--version"),
	]);

	let valid_flags: _ = [
		// "--eval",
		"--help",
		"--test",
		"--version",
	];

	let unique_flags: _ = [
		"--help",
		"--version",
	];

	let mut flags: Vec<String> = vec![];

	for (i, arg) in args.clone().iter().enumerate() {
		println!("[debug] args[{}] = {:?}", i, arg);
		let i: usize = i + 1;

		if arg.len() < 2 || !arg.starts_with('-') {
			break;
		}

		if arg == "-" {
			eprintln!("{}: invalid flag at position {}", error("ArgumentError"), i);
			exit(1);
		}

		let mut flag: &str = &args.remove(0)[..];

		if flag.len() == 2 {
			if !flag_map.contains_key(flag) {
				eprintln!("{}: unknown flag '{}'", error("ArgumentError"), flag);
				exit(1);
			}
			flag = flag_map[flag];
		}
		if !valid_flags.contains(&flag) {
			eprintln!("{}: unknown flag '{}'", error("ArgumentError"), flag);
			exit(1);
		}
		if !unique_flags.contains(&flag) {
			flags.push(flag.to_string());
			continue;
		}
		if i != 1 {
			eprintln!("{}: unexpected flag '{}' at position {}", error("ArgumentError"), flag, i);
			exit(1);
		}
		if flag == "--help" {
			println!("{}", String::new()
				+ "\x1B[90m-h\x1B[0m | \x1B[90m--help   \x1B[0m    Shows this message\n"
				+ "\x1B[90m-v\x1B[0m | \x1B[90m--version\x1B[0m    Prints the current " + NAME + " version");
		} else if flag == "--version" {
			println!("{} {}", NAME, VERSION);
		} else {
			println!("{}: flag '{}' not implemented yet", error("ArgumentError"), flag);
		}
		exit(0);
	}
	// #endregion flags

	if args.is_empty() {
		eprintln!("{}: missing file path", error("ArgumentError"));
		exit(1);
	}

	let file_path: String = args.remove(0);

	(executor, flags, file_path, args)
}