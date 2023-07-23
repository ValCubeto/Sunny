use crate::{id::Id, arguments::Arguments, expressions::Expression};

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Statment {
	Assignment {
		id: Id,
		mutable: bool,
		expr: Expression
	},
	Call {
		id: Id,
		args: Arguments
	}
}