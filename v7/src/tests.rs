use std::rc::Rc;

use crate::{ hashmap, instances::Instance, structs::Struct, id::Id, values::Value, functions::{FunctionValue, Function} };

#[test]
#[ignore]
fn hashmap_macro() {
	dbg!(
		hashmap! {
			test => "Hola",
		},
		hashmap!{crate::about::NAME=>"its the name"}
	);
}

#[test]
#[ignore]
fn instances() {
	let mine = Rc::from(Struct {
		extended: None,
		name: Id::from("hallo"),
		props: hashmap! {
			test => (None, None),
		}
	});
	#[allow(unused)]
	let instance1 = Instance {
		parent: Rc::clone(&mine),
		values: hashmap! {
			test => Value::Id("".into())
		}
	};
	#[allow(unused)]
	let instance2 = Instance {
		parent: Rc::clone(&mine),
		values: hashmap! {
			test => Value::Id("".into())
		}
	};
}

#[test]
fn defined_and_builtin_functions() {
	let defined = FunctionValue::Defined(Function::new(Id::from("hola"), false));
	let builtin = FunctionValue::Builtin(|_args| Value::Id(Id::from("hello")));
	dbg!(defined, builtin); // prints the Function object and the pointer
}