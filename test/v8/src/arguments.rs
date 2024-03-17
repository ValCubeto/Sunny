use crate::{
	aliases::Id,
	values::Value
};

pub struct Argument {
	name: Id,
	value: Value,
	// type: Rc<Struct>
}

// positional, named, rest
// only one rest argument