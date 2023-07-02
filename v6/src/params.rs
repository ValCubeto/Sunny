use std::rc::Rc;

use crate::dict::Key;
use crate::types::{Type, Value};

pub type Param = (Key, (Rc<Type>, Rc<Value>));

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
	pub fn add(&mut self, param: Rc<str>) {
		for key
	}
}