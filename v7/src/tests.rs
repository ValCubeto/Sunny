use std::rc::Rc;

use crate::{ hashmap, instances::Instance, structs::Struct, id::Id, values::Value };

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
	let instance1 = Instance {
		parent: Rc::clone(&mine),
		values: hashmap! {
			test => Value::Id("".into())
		}
	};
	let instance2 = Instance {
		parent: Rc::clone(&mine),
		values: hashmap! {
			test => Value::Id("".into())
		}
	};
	// dbg!(
	// 	&instance2.parent.name,
	// 	&instance1.parent.name,
	// 	&mine.name
	// );
}