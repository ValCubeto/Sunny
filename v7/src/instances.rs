use std::{collections::HashMap, rc::Rc};
use crate::{structs::Struct, values::Value, id::Id};

pub struct Instance {
	pub parent: Rc<Struct>,
	pub values: HashMap<Id, Value>
}