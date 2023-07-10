use crate::{
	dict::{Dict, Id},
	ns_parser::Namespace,
	func_parser::Function,
	params::Index,
	structs::{Struct, Instance},
};

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Value {
	None,
	String(String),

	i8(i8),
	i16(i16),
	i32(i32),
	i64(i64),
	i128(i128),
	isize(isize),
	u8(u8),
	u16(u16),
	u32(u32),
	u64(u64),
	u128(u128),
	usize(usize),
	f32(f32),
	f64(f64),

	List(Vec<Value>),
	Dict(Dict),
	Namespace(Namespace),
	Function(Function),
	Struct(Struct),
	Instance(Instance),
}

impl From<String> for Value {
	fn from(value: String) -> Self {
		Value::String(value)
	}
}

impl From<i8> for Value {
	fn from(value: i8) -> Self {
		Value::i8(value)
	}
}

impl<T> From<Vec<T>> for Value where Value: From<T> {
	fn from(value: Vec<T>) -> Self {
		Value::List(value.iter().map(|v| { Value::from(*v) }).collect())
	}
}

#[derive(Debug)]
enum Test {
	Builtin(fn(Vec<(Index, Value)>) -> bool),
	Defined(Box<Function>)
}

#[derive(Debug)]
pub struct Type {
	test: Test,
	name: Id
}

impl Type {
	pub fn any_or_none() -> Type {
		Type {
			name: Id::from("any?"),
			test: Test::Builtin(|_args| {
				true
			})
		}
	}
}