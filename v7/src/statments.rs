use crate::{id::Id, arguments::Arguments, expressions::Expression};

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Statment {
	Assignment {
		id: Id,
		mutable: bool,
		value: Expression
	},
	Call {
		id: Id,
		args: Arguments
	}
}