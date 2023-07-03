use crate::dict::Key;
use crate::types::{Type, Value};

#[derive(Debug)]
pub enum Index {
	Numeric(usize),
	Named(Key)
}

pub type Param = (Key, (Box<Type>, Box<Value>));

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
	pub fn add(&mut self, param: Box<str>) {
		for (key, _) in self.vec.iter() {
			key.0;
		}
	}
}