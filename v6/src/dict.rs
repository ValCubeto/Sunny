use std::collections::HashMap;
use std::rc::Rc;
use crate::types::Value;

pub type Key = Rc<str>;

pub struct Dict {
	map: HashMap<Key, Value>
}

impl/* <const N: usize> *//*  From<[(Key, Value); N]> for  */Dict {
	pub fn new(src: [(Key, Value)]) -> Self {
		Dict { map: HashMap::from(src) }
	}
}