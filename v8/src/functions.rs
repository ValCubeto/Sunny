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

enum FunctionValue {
	Builtin(fn(Arguments) -> Result<Value, (Id, Id)>),
	Defined(Vec<Statment>)
}

impl Function {
	pub fn call(&self, args: Arguments) -> Value {
		let r = match self.value {
			FunctionValue::Builtin(func) => match func(args) {
				Ok(v) => v,
				Err((err_name, description)) => error!(err_name, "{description}")
			},
			_ => todo!("")
		};
		println!("todo");
		Value::None
	}
}
