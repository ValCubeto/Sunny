use std::env::args_os;
use std::collections::HashMap;
use std::process::exit;
use crate::about::{NAME, VERSION};
use crate::colors::gray;
use crate::errors::{ArgumentError, debug};

pub fn parse() -> (String, Vec<String>, String, Vec<String>) {
	let mut flags: Vec<String> = vec![];
	let mut args: Vec<String> = vec![];

	for arg in args_os() {
		args.push(arg.to_string_lossy().to_string());
	}

	debug!("args = {:?}", args);

	let exec_path: String = args.remove(0);

	// #region flags
	let valid_flags: _ = [
		// "-e",
		"--help",
		"--test",
		"--version",
	];

	let valid_flags_short: _ = [
		"-h",
		"-t",
		"-v",
	];

	let unique_flags: _ = [
		"--help",
		"--version",
	];

	// env::args() panics
	for (i, arg) in args.clone().iter().enumerate() {
		// one-based
		let i: usize = i + 1;

		// is not a flag
		if arg.len() < 2 || !arg.starts_with('-') {
			break;
		}

		args.remove(0);
		let mut flag: &str = arg.as_str();

		if flag.len() == 2 {
			// no hay ganas de buscar un find_index
			for (i, short) in valid_flags_short.clone().iter().enumerate() {
				let short: &str = *short;
				if flag == short {
					debug!("\"{}\" -> \"{}\"", gray(flag), gray(valid_flags[i]));
					flag = valid_flags[i];
				}
			}
		}
		if !valid_flags.contains(&flag) {
			ArgumentError!("unknown flag '{}'", flag);
		}
		if !unique_flags.contains(&flag) {
			flags.push(flag.to_string());
			continue;
		}
		if i != 1 {
			ArgumentError!("unexpected flag \"{}\" at position {}", flag, i);
		}
		match flag {
			"--help" => {
				let descriptions: HashMap<&str, String> = HashMap::from([
					("--help", "Shows this message".to_string()),
					("--version", format!("Prints the current {} version", NAME))
				]);

				let max_flag_len: usize = match descriptions.keys()
					.max_by(|a: &&&str, b: &&&str| { a.len().cmp(&b.len()) })
				{
					None => exit(1),
					Some(n) => n
				}.len();

				for (flag, description) in descriptions {
					println!("{}{}   {}", gray(flag), " ".repeat(max_flag_len - flag.len()), description);
				};
				exit(0);
			}
			"--version" => {
				println!("{} {}", NAME, VERSION);
			}
			_ => {
				ArgumentError!("flag '{}' not implemented yet", flag);
			}
		}
	}
	// #endregion flags

	if args.is_empty() {
		ArgumentError!("missing file path");
	}

	let file: String = args.remove(0);

	(exec_path, flags, file, args)
}