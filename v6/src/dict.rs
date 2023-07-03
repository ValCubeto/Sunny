use std::collections::HashMap;
use std::fmt::Display;
use crate::types::Value;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Key(pub Box<str>);

impl From<&str> for Key {
	fn from(string: &str) -> Self {
		Key(Box::from(string))
	}
}

impl Display for Key {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl Key {
	pub fn unwrap(&self) -> &str {
		&(self.0)
	}
}

pub struct Dict(pub HashMap<Key, Value>);

impl<const N: usize> From<[(Key, Value); N]> for Dict {
	fn from(src: [(Key, Value); N]) -> Self {
		Dict(HashMap::from(src))
	}
}