use crate::{
	dict::{Dict, Key},
	ns_parser::Namespace,
	func_parser::Function,
	params::Index, structs::Struct,
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
	Instance(Key, Vec<Value>),
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