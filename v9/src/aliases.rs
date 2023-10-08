use std::{ rc::Rc, collections::HashMap };
use crate::values::Value;

pub type Id = Rc<str>;
pub type ArrayPtr = Rc<Vec<Value>>;
pub type DictPtr = Rc<HashMap<Id, Value>>;
