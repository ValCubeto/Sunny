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
			Assignment { id, mutable, value } => {
				global.set(id.clone(), resolve(value))
			},
			_ => InternalError!("not implemented")
		}
	}
	Value::None
}

pub fn resolve(tokens: &Vec<Token>) -> Value {
	let value: Value;
	for token in tokens {
		value = match token {
			Token::Value(v) => {
				value = v.clone();
				break;
			}
			_ => InternalError!("todo")
		}
	}
	value
}