use std::process::exit;
use crate::colors::error;

pub fn parse_toml(string: String) -> toml::map::Map<String, toml::Value> {
	match string.parse::<toml::Table>() {
		Err(err) => {
			eprintln!("{}: {}", error("LoadError"), err);
			exit(1);
		}
		Ok(table) => table
	}
}