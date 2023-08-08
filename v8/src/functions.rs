use std::rc::Rc;
use crate::{
	aliases::{Id, Arguments},
	values::Value, expressions::Expression
};

struct Function {
	name: Id,
	params: Arguments,
	/// Example:
	/// ```sunny
	/// fun test() {
	///     const a = 5
	///     const lamb = (n) => n * a
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