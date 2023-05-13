use std::fs::read_to_string;
use std::process::exit;

pub fn read(path: String) -> String {
	match read_to_string(path) {
		Err(err) => {
			eprintln!("{}", err);
			exit(1);
		}
		Ok(file) => file
	}
}