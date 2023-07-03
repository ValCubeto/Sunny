use crate::dict::Key;
use crate::types::{Type, Value};

pub type Param = (Key, (Box<Type>, Box<Value>));

#[allow(unused)]
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