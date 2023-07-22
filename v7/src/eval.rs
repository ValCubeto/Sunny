use crate::{ values::Value,
	arguments::Arguments,
	statments::Statment,
	errors::InternalError,
	context::Context,
	expressions::Expression };

pub fn eval_ast(ast: &Vec<Statment>, additional_data: Arguments, Context { stack, .. }: &mut Context) -> Value {
	for statment in ast {
		use Statment::*;
		match statment {
			Assignment { id, mutable, value: tokens } => {
				let value = resolve(tokens);
				println!("set {} = {:?}", id, value);
				stack.set(id.clone(), value)
			},
			_ => InternalError!("not implemented")
		}
	}
	Value::None
}

pub fn resolve(tokens: &Vec<Expression>) -> Value {
	let mut value: Value = Value::None;
	#[allow(unreachable_code)]
	for token in tokens {
		value = match token {
			Expression::Value(v) => {
				value = v.clone();
				break;
			}
			_ => InternalError!("to-do")
		}
	}
	value
}