use std::rc::Rc;
use crate::dict::Dict;
use crate::ns_parser::Namespace;
use crate::func_parser::Function;

#[allow(unused)]
pub enum Value {
	None,
	String(String),
	List(Vec<Value>),
	Dict(Dict),
	Namespace(Namespace),
	Function(Function)
}

enum Test {
	Builtin(fn(Vec<(String, Value)>) -> bool),
	Defined(Box<Function>)
}

pub struct Type {
	test: Test,
	name: Box<str>
}