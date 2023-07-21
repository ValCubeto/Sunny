use crate::{context::Context, values::Value, errors::SyntaxError};

pub fn parse_expr(ctx: &mut Context) -> Vec<Token> {
	let mut tokens = Vec::new();
	ctx.go();
	match ctx.current {
		'"' | '\'' => {
			let quote = ctx.current;
			ctx.next_char();
			let mut string = String::new();
			while ctx.current != quote {
				string.push(ctx.current);
				ctx.next_char();
			}
			dbg!(&string);
			tokens.push(Token::Value(Value::String(string)));
		}
		_ => SyntaxError!(ctx, "unexpected character {:?}", ctx.current)
	}
	// order tokens
	// SyntaxError!(ctx, "w");
	tokens
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Expression {
	Token(Box<Token>),
	Value(Value)
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Token {
	Value(Value),
	Sum(Expression, Expression),
	Sub(Expression, Expression),
	Mul(Expression, Expression),
	Div(Expression, Expression),
}