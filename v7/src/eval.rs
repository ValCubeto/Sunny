use crate::{
	values::Value,
	aliases::Arguments,
	statments::Statment,
	context::Context,
	expressions::Expression,
	stack::Stack as _,
	{ internal_error, syntax_error },
};

#[allow(unused)]
pub fn eval_ast(ast: &Vec<Statment>, additional_data: Arguments, Context { stack, .. }: &mut Context) -> Value {
	for statment in ast {
		use Statment::*;
		match statment {
			Assignment { id, mutable, expr } => {
				let value = resolve(expr);
				println!("set {} = {:?}", id, value);
				stack.set_value(id.clone(), *value);
			},
			_ => internal_error!("not implemented")
		}
	}
	Value::None
}

pub fn resolve(expr: &Expression) -> &Value {
	#[allow(unused)]
	let mut value: &Value = &Value::None;
	match expr {
		Expression::Value(v) => {
			value = v
		}
		_ => syntax_error!("to-do")
	}
	value
}