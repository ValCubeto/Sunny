use std::collections::HashMap;

use crate::mod_parser::Module;
use crate::func_parser::Function;

#[allow(unused)]
#[derive(Debug)]
pub enum Value {
	String(String),
	List(Vec<Value>),
	Dict(HashMap<String, Value>),
	Module(Module),
	Function(Function)
}