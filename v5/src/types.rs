use std::collections::HashMap;

pub type Dict = HashMap<&'static str, Any>;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Any {
	None,
	Infinity,
	Bool(bool),
	List(Vec<Any>),
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

impl Any {
	pub fn unwrap_bool(&self) -> bool {
		match self { Any::Bool(value) => *value, _ => panic!("unwrapping on incorrect variant") }
	}
}

fn a() {
	let a = Any::Bool(true).unwrap_bool();
	let b = Any::Bool; // ?????
}