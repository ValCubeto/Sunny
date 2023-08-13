use std::collections::HashMap;

use crate::{id::Id, values::Value, hashmap, functions::{Function, FunctionValue}};

pub fn make_global() -> HashMap<Id, Value> {
	hashmap! {
		println => Value::Function(Box::new(Function { name: Id::from("println"), value: FunctionValue::Builtin(|args| {
			for arg in &args {
				println!("println: {:?}", arg);
			}
			Ok(Value::None)
		})}))
		/* 
		make_function!(println; |args| {})
		make_function!(println; vec![])
		 */
	}
}