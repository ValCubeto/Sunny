use crate::{id::Id, arguments::Arguments, expressions::Token};

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Statment {
	Assignment {
		id: Id,
		mutable: bool,
		value: Vec<Token>
	},
	Call {
		id: Id,
		args: Arguments
	}
}