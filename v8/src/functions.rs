use std::rc::Rc;
use crate::{
	aliases::{ Id, Arguments },
	values::Value, statments::Statment, error
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

struct Error {
	name: Id,
	description: Id
}

enum FunctionValue {
	Builtin(fn(Arguments) -> Result<Value, Error>),
	Defined(Vec<Statment>)
}

impl Function {
	pub fn call(&self, args: Arguments) -> Value {
		let r = match self.value {
			FunctionValue::Builtin(func) => match func(args) {
				Ok(v) => v,
				Err(err) => error!(err.name, "{}", err.description)
			},
			_ => todo!("")
		};
		println!("todo");
		Value::None
	}
}
