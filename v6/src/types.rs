use crate::{
	dict::{Dict, Key},
	ns_parser::Namespace,
	func_parser::Function,
	params::Index,
};

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
	name: Key
}

impl Type {
	pub fn any_or_none() -> Type {
		Type {
			name: Key::from("any?"),
			test: Test::Builtin(|_args| {
				true
			})
		}
	}
}