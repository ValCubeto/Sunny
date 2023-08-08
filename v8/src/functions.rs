use std::rc::Rc;
use crate::{
	aliases::Id,
	values::Value
};

struct Function {
	name: Id,
	params: Arguments,
	/// Example:
	/// ```sunny
	/// fun test() {
	///     const a = 5
	///     const lamb = (n) => n * 5
	///     return lamb
	/// }
	/// ```
	/// Here, the value of `a` is needed until the lambda exists
	dependencies: Vec<Rc<Value>>,
	value: FunctionValue
}

enum FunctionValue {
	Builtin(fn(Arguments) -> Result<Value, Id>),
	Defined(Vec<Statment>)
}

enum Statment {
	If {
		condition: Expression,
		body: Vec<Statment>,
		else_body: Vec<Statment>
	},
	Assignment {
		id: Vec<Id>,
		value: Expression
	},
	Call {
		id: Vec<Id>,
		args: Arguments
	}
}