use std::collections::BTreeMap;
use crate::{types::{Type, Value}, errors::InternalError};

pub type Params = BTreeMap<String, (Type, Option<Value>)>;

#[derive(Clone)]
pub enum Statment {
	Declaration(Vec<String>, (Type, Value)),
	Call(Vec<String>, BTreeMap<String, Value>),
	If(Expression, CodeBlock, CodeBlock)
}

type Expression = Vec<String>;

type CodeBlock = Vec<Statment>;

pub struct Function {
	pub name: String,
	pub params: Params, // sorted keys for indexing
	pub rettype: Type,
	pub body: CodeBlock
}

pub fn parse_function(chars: &[char], i: &mut usize) -> Function {
	let mut name: String = String::new();
	let mut p: Params = Params::from([
		("name".to_string(), (Type::Builtin(&|p: Params| true), Some(Value::Bool(true))))
	]);
	match p.get_mut(&"name".to_string()) { Some(v) => v, _ => {InternalError!("");} }
	.1 = Some(Value::F32(1f32));
	Function {
		name,
		params: Params::new(),
		rettype: Type::Builtin(&|params: Params| { true }),
		body: vec![]
	}
}