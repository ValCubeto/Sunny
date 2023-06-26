use std::collections::HashMap;

use crate::ns_parser::Namespace;
use crate::func_parser::Function;

#[allow(unused)]
#[derive(Debug)]
pub enum Value {
	String(String),
	List(Vec<Value>),
	Dict(HashMap<String, (Type, Value)>),
	Namespace(Namespace),
	Function(Function)
}

enum Test {
	Builtin(dyn Fn(Vec<(String, Value)>) -> bool),
	Defined(Function)
}

pub struct Type {
	test: Test
}