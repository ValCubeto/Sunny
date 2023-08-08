use std::rc::Rc;
use crate::{
	aliases::{ Id, Arguments },
	values::Value, statments::Statment
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
