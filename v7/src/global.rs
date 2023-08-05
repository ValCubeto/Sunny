use std::collections::HashMap;

use crate::{id::Id, values::Value, hashmap, functions::FunctionValue};

pub fn make_global() -> HashMap<Id, Value> {
	hashmap! {
		Id::from("println") => Value::Function(Box::new(FunctionValue::Builtin(|args| {
			Value::None
		})))
	}
}