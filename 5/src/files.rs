use std::fs::read_to_string;
use crate::errors::LoadError;

pub fn read(path: String) -> String {
	match read_to_string(path) {
		Err(err) => {
			LoadError!("{}", err);
		}
		Ok(file) => file
	}
}