use std::{ rc::Rc, collections::HashMap };
use crate::values::Value;

pub type Id = Rc<str>;
pub type Array = Rc<Vec<Value>>;
#[allow(unused)]
pub type ConstArray = Rc<[Value]>;
pub type Dict = Rc<HashMap<Id, Value>>;