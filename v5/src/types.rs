use std::collections::HashMap;
use crate::functions::{Params, Statment};

#[derive(Clone)]
pub enum Type {
	Builtin(&'static dyn Fn(Params) -> bool),
	Defined(Vec<Statment>)
}

pub type Dict = HashMap<String, (Type, Value)>;
pub type List = Vec<(Type, Value)>;

#[allow(unused)]
#[derive(Clone)]
pub enum Value {
	None,
	Infinity,
	Bool(bool),
	List(Vec<Value>),
	Dict(Dict),
	String(String),
	Namespace(Dict),
	// Function(Ast),

	U8(u8),
	U16(u16),
	U32(u32),
	U64(u64),
	U128(u128),
	Usize(usize),
	I8(i8),
	I16(i16),
	I32(i32),
	I64(i64),
	I128(i128),
	Isize(isize),
	F32(f32),
	F64(f64),

	Number(String),
}