use crate::{id::Id, arguments::Arguments, values::Value};

#[allow(unused)]
#[derive(Debug)]
pub enum Statment {
	Declaration(Id, Value),
	Call(Id, Arguments)
}