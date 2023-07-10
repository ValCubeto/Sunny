use crate::dict::Key;
use crate::types::{Type, Value};

#[allow(unused)]
#[derive(Debug)]
pub enum Index {
	Numeric(usize),
	Named(Key)
}

#[allow(unused)]
#[derive(Debug)]
pub struct Param {
	name: Key,
	r#type: Option<Box<Type>>,
	value: Option<Box<Value>>
}

impl Param {
	pub fn new(name: Key, r#type: Option<Box<Type>>, value: Option<Box<Value>>) -> Self {
		Param {
			name,
			r#type,
			value
		}
	}
}

#[allow(unused)]
#[derive(Debug)]
pub struct Params {
	vec: Vec<Param>,
	rest: Option<(usize, Param)>
}

impl Params {
	pub fn new() -> Self {
		Params {
			vec: Vec::new(),
			rest: None
		}
	}
	pub fn add(&mut self, k: Key, v: (Box<Type>, Box<Value>)) {
		for Param { name, .. } in self.vec.iter() {
			todo!();
		}
	}
}