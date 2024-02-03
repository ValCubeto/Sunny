use {
	std::{
		rc::Rc,
		collections::HashMap
	},
	crate::{
		values::Value,
		expressions::Expression
	}
};

pub type Id = Rc<str>;
pub type Dict = HashMap<Id, Value>;
pub type Arguments = Rc<[Expression]>;
