use std::fmt::Debug;

use crate::{id::Id, values::Value, namespaces::Namespace};

pub struct Stack {
	pub vec: Vec<Namespace>
}

impl Debug for Stack {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.vec.fmt(f)
	}
}

impl Stack {
	pub fn new() -> Self {
		Stack {
			vec: Vec::new()
		}
	}
	pub fn get(&self, id: &Id) -> Option<&Value> {
		for space in &(self.vec) {
			let value = space.get(id);
			if value.is_some() {
				return value
			}
		}
		None
	}
	pub fn set(&mut self, id: Id, value: Value) {
		self.vec
			.last_mut()
			.unwrap()
			.set(id, value);
	}
}