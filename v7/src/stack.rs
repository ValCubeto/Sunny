use crate::{id::Id, values::Value, namespaces::Namespace};

pub trait Stack {
	fn get(&self, id: &Id) -> Option<&Value>;
	fn set(&mut self, id: Id, value: Value);
}

impl Stack for Vec<Namespace> {
	fn get(&self, id: &Id) -> Option<&Value> {
		for space in self {
			let value = space.get(id);
			if value.is_some() {
				return value
			}
		}
		None
	}
	fn set(&mut self, id: Id, value: Value) {
		self
			.last_mut()
			.unwrap()
			.set(id, value);
	}
}