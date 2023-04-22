use std::collections::HashMap;
use std::env::args_os;
use std::path::PathBuf;
use std::process::exit;

struct Error {} impl Error {
	fn argument(message: Vec<&str>) {}
}

pub fn main() {
	Error::argument(vec!["hola"])
}

pub fn analyze(args: Vec<String>) -> (Vec<String>, String, Vec<String>) {
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
	};
	(flags, relative, args)
}