use std::rc::Rc;
use crate::dict::{Dict, Key};
use crate::ns_parser::Namespace;
use crate::func_parser::Function;
use crate::params::Index;

#[derive(Debug)]
pub enum Value {
	None,
	String(String),
	List(Vec<Value>),
	Dict(Dict),
	Namespace(Namespace),
	Function(Function)
}

#[derive(Debug)]
enum Test {
	Builtin(fn(Vec<(Index, Value)>) -> bool),
	Defined(Box<Function>)
}

#[derive(Debug)]
pub struct Type {
	test: Test,
	name: Box<str>
}

impl Type {
	pub fn any_or_none() -> Type {
		Type {
			name: Box::from(""),
			test: Test::Builtin(|Arguments|)
		}
	}
}