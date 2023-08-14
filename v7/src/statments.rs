use crate::{
	aliases::{ Id, Arguments },
	expressions::Expression
};

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statment {
	Assignment {
		id: Id,
		mutable: bool,
		expr: Expression
	},
	Call {
		// FIXME: add Vec<Id>
		id: Id,
		args: Arguments
	}
}