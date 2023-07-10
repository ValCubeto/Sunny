use std::collections::HashMap;
use std::fmt::Display;
use crate::types::{Value, IntoSunnyValue};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Id(pub Box<str>);

impl From<&str> for Id {
	fn from(string: &str) -> Self {
		Id(Box::from(string))
	}
}

impl Display for Id {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl Id {
	pub fn as_str(&self) -> &str {
		&(self.0)
	}
}

#[derive(Debug)]
pub struct Dict(pub HashMap<Id, Value>);

impl<T, const N: usize> From<[(&str, T); N]> for Dict where Value: From<T> {
	fn from(src: [(&str, T); N]) -> Self {
		Dict(
			HashMap::from(src.map(|(k, v)| {
				(Id::from(k), Value::from(v))
			}))
		)
	}
}