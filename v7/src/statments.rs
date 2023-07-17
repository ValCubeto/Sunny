use crate::{id::Id, arguments::Arguments, values::Value, expressions::Expression};

#[allow(unused)]
#[derive(Debug)]
pub enum Statment {
	Declaration(Id, bool, Expression),
	Call(Id, Arguments)
}