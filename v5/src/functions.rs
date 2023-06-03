use std::collections::BTreeMap;
use crate::types::{Type, Value};
use crate::errors::InternalError;
use crate::chrono_map::ChronoMap;

pub type Params = ChronoMap<String, (Type, Value)>;
type Expression = Vec<String>;
type CodeBlock = Vec<Statment>;

// #[derive(Clone)]
#[allow(unused)]
pub enum Statment {
	Declaration(Vec<String>, (Type, Value)),
	Call(Vec<String>, BTreeMap<String, Value>),
	If(Expression, CodeBlock, CodeBlock)
}


pub struct Function {
	pub name: String,
	pub params: Params,
	pub rettype: Type,
	pub body: CodeBlock
}

#[allow(unused)]
pub fn parse_function(chars: &[char], i: &mut usize) -> Function {
	let mut name: String = String::new();
	std::collections::BTreeMap::from([("a".to_string(), Value::Bool(false))]);
	let mut p: Params = ChronoMap::from([
		("name".to_string(), (Type::Builtin(&|p: Params| true), Value::Bool(true)))
	]);
	match p.get_mut(&"name".to_string()) { Some(v) => v, _ => { InternalError!(""); } }.1 = Value::F32(1f32);
	Function {
		name,
		params: Params::new(),
		rettype: Type::Builtin(&|params: Params| { true }),
		body: vec![]
	}
}