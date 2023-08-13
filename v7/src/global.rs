use std::collections::HashMap;
use crate::{
	id::Id,
	values::Value,
	functions::{ Function, FunctionValue },
	{ hashmap, builtin_function }
};

pub fn make_global() -> HashMap<Id, Value> {
	hashmap! {
		println => builtin_function!("println", |args| {
			for arg in &args {
				println!("println: {:?}", arg);
			}
			Ok(Value::None)
		})
	}
}