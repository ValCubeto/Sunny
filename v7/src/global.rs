use std::collections::HashMap;

use crate::{id::Id, values::Value, hashmap};

pub fn make_global() -> HashMap<Id, Value> {
	hashmap! {
		"println" => Value::Function()
	}
}