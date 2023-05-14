use crate::errors::LoadError;

pub fn parse_toml(string: String) -> toml::map::Map<String, toml::Value> {
	match string.parse::<toml::Table>() {
		Err(err) => {
			LoadError!("{}", err);
		}
		Ok(table) => table
	}
}