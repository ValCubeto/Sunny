use crate::{context::Context, values::Value, errors::SyntaxError};

pub fn parse_expr(ctx: &mut Context) -> Token {
	let mut expressions = Vec::new();
	match ctx.current {
		'"' | '\'' => {
			let quote = ctx.current;
			ctx.next_char();
			let mut string = String::new();
			while ctx.current != quote {
				string.push(ctx.current);
				ctx.next_char();
			}
			expressions.push(Expression::Value(Value::String(string)));
		}
		_ => SyntaxError!(ctx, "unknown character {:?}", ctx.current)
	}
	// order tokens
	// SyntaxError!(ctx, "w");
	let mut token = Token::Value(expressions[0]);
	token
}

#[allow(unused)]
#[derive(Debug)]
pub enum Expression {
	Token(Box<Token>),
	Value(Value)
}

#[allow(unused)]
#[derive(Debug)]
pub enum Token {
	Value(Value),
	Sum(Expression, Expression),
	Sub(Expression, Expression),
	Mul(Expression, Expression),
	Div(Expression, Expression),
}