use std::collections::HashMap;

use crate::ns_parser::Namespace;
use crate::func_parser::Function;

#[allow(unused)]
#[derive(Debug)]
pub enum Value {
	String(String),
	List(Vec<Value>),
	Dict(HashMap<String, Value>),
	Namespace(Namespace),
	Function(Function)
}