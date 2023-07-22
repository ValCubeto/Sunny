use crate::{ values::Value,
	arguments::Arguments,
	statments::Statment,
	errors::InternalError,
	context::Context,
	expressions::Token };

pub fn eval_ast(ast: &Vec<Statment>, additional_data: Arguments, Context { global, .. }: &mut Context) -> Value {
	for statment in ast {
		use Statment::*;
		match statment {
			Assignment { id, mutable, value: tokens } => {
				let value = resolve(tokens);
				println!("set {} = {:?}", id, value);
				global.set(id.clone(), value)
			},
			_ => InternalError!("not implemented")
		}
	}
	Value::None
}

pub fn resolve(tokens: &Vec<Token>) -> Value {
	let mut value: Value = Value::None;
	#[allow(unreachable_code)]
	for token in tokens {
		value = match token {
			Token::Value(v) => {
				value = v.clone();
				break;
			}
			_ => InternalError!("to-do")
		}
	}
	value
}