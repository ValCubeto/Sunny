use std::{collections::HashMap, rc::Rc};

use crate::{id::Id, values::Value};

#[derive(PartialEq, Eq)]
pub struct Struct {
	pub extended: Option<Vec<Rc<Struct>>>,
	pub name: Id,
	pub props: HashMap<Id, (Option<Rc<Struct>>, Option<Value>)>
}