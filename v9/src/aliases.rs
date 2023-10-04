use std::{ rc::Rc, collections::HashMap };
use crate::values::Value;

pub type Id = Rc<str>;
// Rc<[Value]> has 2 extra bytes for some reason
pub type ArrayPtr = Rc<Vec<Value>>;
pub type DictPtr = Rc<HashMap<Id, Value>>;
