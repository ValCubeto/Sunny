use crate::{
	aliases::{ Id, Arguments },
	expressions::Expression
};

pub enum Statment {
	Assignment {
		id: Vec<Id>,
		value: Expression
	},
	Call {
		id: Vec<Id>,
		args: Arguments
	},
	If {
		condition: Expression,
		body: Vec<Statment>,
		else_body: Vec<Statment>
	},
}