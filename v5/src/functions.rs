use std::collections::BTreeMap;
use crate::types::{Type, Value};

pub type Params = BTreeMap<String, (Type, Value)>;

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
	Function {
		name,
		params: Params::new(),
		rettype: Type::Builtin(&|params| { true }),
		body: vec![]
	}
}