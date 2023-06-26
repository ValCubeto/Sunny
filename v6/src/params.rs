use crate::types::Value;

#[allow(unused)]
#[derive(Debug)]
pub struct Params {
	vec: Vec<(String, (Type, Value))>,
	rest: (usize, String, (Type, Value))
}

impl Params {
	pub fn new() -> Self {
		Params {
			vec: Vec::new()
		}
	}
	pub fn add(&mut self, )
}