use std::env::args_os as get_args_os;
use std::collections::HashMap;
use std::process::exit;
use crate::about::{NAME, VERSION};

fn error(text: &str) -> String {
	format!("\u{1b}[31m\u{1b}[1m{}\u{1b}[22m\u{1b}[39m", text)
}

pub fn parse() -> (String, Vec<String>, String, Vec<String>) {
	let args_os: _ = get_args_os();
	let mut flags: Vec<String> = vec![];
	let mut args: Vec<String> = vec![];

	let exec_path: String = args.remove(0);

	println!("[debug] exec_path = {:?}", exec_path);

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

	for (i, arg) in args_os.enumerate() {
		// env::args() panics
		let arg: String = arg.to_string_lossy().to_string();
		println!("[debug] args[{}] = {:?}", i, arg);
		let i: usize = i + 1;
		
		if arg.len() < 2 || !arg.starts_with('-') {
			break;
		}

		if arg == "-" {
			eprintln!("{}: invalid flag at position {}", error("ArgumentError"), i);
			exit(1);
		}

		let mut flag: &str = &arg[..];

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
		match flag {
			"--help" => {
				println!("{}", String::new()
					+ "\x1B[90m-h\x1B[0m | \x1B[90m--help   \x1B[0m    Shows this message\n"
					+ "\x1B[90m-v\x1B[0m | \x1B[90m--version\x1B[0m    Prints the current " + NAME + " version");
			}
			"--version" => {
				println!("{} {}", NAME, VERSION);
			}
			_ => {
				println!("{}: flag '{}' not implemented yet", error("ArgumentError"), flag);
			}
		}
		exit(0);
	}
	// #endregion flags
	if args_os.len() != 0 {
		for arg in args_os {
			args.push(arg.to_string_lossy().to_string());
		}
	}

	if args.is_empty() {
		eprintln!("{}: missing file path", error("ArgumentError"));
		exit(1);
	}

	let file: String = args.remove(0);

	(exec_path, flags, file, args)
}