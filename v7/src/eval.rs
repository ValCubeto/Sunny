use crate::{ values::Value,
	arguments::Arguments,
	statments::Statment,
	InternalError, SyntaxError,
	context::Context,
	expressions::Expression,
	stack::Stack };

pub fn eval_ast(ast: &Vec<Statment>, additional_data: Arguments, Context { stack, .. }: &mut Context) -> Value {

	for statment in ast {
		use Statment::*;
		match statment {
			Assignment { id, mutable, expr } => {
				let value = resolve(expr);
				println!("set {} = {:?}", id, value);
				stack.set(id.clone(), value)
			},
			_ => InternalError!("not implemented")
		}
	}
	Value::None
}

pub fn resolve(expr: &Expression) -> Value {
	let mut value: Value = Value::None;
	match expr {
		Expression::Value(v) => {
			value = v.clone()
		}
		_ => SyntaxError!("to-do")
	}
	value
}