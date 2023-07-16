use crate::id::Id;
use std::collections::HashMap;

#[allow(unused)]
pub enum Value {
	None,
	String(String),
	Id(Id),
	Vec(Vec<Value>),
	Dict(Box<HashMap<Id, Value>>)
}