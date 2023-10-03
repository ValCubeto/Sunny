use std::{ rc::Rc, collections::HashMap };
use crate::values::Value;

pub type Id = Rc<str>;
pub type Array = Rc<Vec<Value>>;
// pub type ConstArray = Rc<[Value]>;    // 2 extra bytes?
pub type Dict = Rc<HashMap<Id, Value>>;
