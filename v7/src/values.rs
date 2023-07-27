use crate::{id::Id, functions::Function, namespaces::Namespace};
use std::collections::HashMap;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Value {
	None,
	String(String),
	Id(Id),
	Vec(Vec<Value>),
	Dict(HashMap<Id, Value>),
	Function(Box<Function>),
	Namespace(Box<Namespace>),
	// Struct(Box<Struct>),
	// Instance(Box<Instance>),
}

impl Value {
	pub fn typename(&self) -> &str {
		use Value::*;
		match self {
			None => "none",
			String(_) | Id(_) => "string",
			Vec(_) => "vector",
			Dict(_) => "dictionary",
			Function(_) => "function",
			Namespace(_) => "namespace",
			// Struct(_) => "struct"
			// Instance(obj) => format!("instance of {}", obj.parent).as_str()
		}
	}
}