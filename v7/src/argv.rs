use {
	std::{
		path::PathBuf,
		process::exit 
	},
	crate::{
		about::{ NAME, VERSION },
		aliases::Id,
		{ internal_error, argument_error }
	}
};

#[derive(Debug)]
pub struct ParsedArgs {
	pub exec_path: PathBuf,
	pub flags: Vec<Id>,
	pub main_path: PathBuf,
	pub args: Vec<Id>
}

pub fn parse_args() -> ParsedArgs {
	let mut raw_args = std::env::args_os().enumerate();

	let exec_path: PathBuf = match raw_args.next() {
		Some((_i, os_string)) => PathBuf::from(os_string),
		None => internal_error!("argv is empty")
	};

	let mut flags: Vec<Id> = Vec::new();

	let mut main_path = None;

	for (i, raw_arg) in &mut raw_args {
		let flag: &str = match raw_arg.to_str() {
			Some(flag) => flag,
			None => argument_error!("the argument at position {} has some invalid unicode", i + 1)
		};

		if !flag.starts_with('-') {
			main_path = Some(PathBuf::from(raw_arg));
			break;
		}

		match flag {
			"-v" | "--version" => {
				let arg_count = raw_args.len();
				if arg_count > 0 {
					println!("# unused {} extra arguments", arg_count);
				}
				println!("{NAME} {VERSION}");
				exit(0);
			}
			"--test" => flags.push(Id::from(flag)),
			_ => argument_error!("invalid flag {:?}", flag)
		}
	}

	if main_path.is_none() {
		// TODO: interactive mode
		argument_error!("missing file path");
	}

	let mut args: Vec<Id> = Vec::new();

	for (i, arg) in &mut raw_args {
		args.push(match arg.to_str() {
			Some(arg) => Id::from(arg),
			None => argument_error!("the argument at position {} has some invalid unicode", i + 1)
		})
	}

	ParsedArgs {
		exec_path,
		flags,
		main_path: main_path.unwrap(), // unwrap_or(PathBuf::from("<stdin>"))
		args
	}
}