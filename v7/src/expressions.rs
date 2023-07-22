use crate::{context::Context, values::Value, errors::SyntaxError, id::Id};

pub fn parse_expr(ctx: &mut Context) -> Vec<Token> {
	let mut tokens = Vec::new();
	ctx.go();
	match ctx.current {
		'"' | '\'' => {
			let string = collect_string(ctx);
			ctx.skip_spaces();

			match ctx.current {
				'\n' => tokens.push(Token::Value(Value::String(string))),
				_ => SyntaxError!(ctx, "unexpected character {:?}", ctx.current)
			}
		}
		_ => SyntaxError!(ctx, "unexpected character {:?}", ctx.current)
	}
	// order tokens
	// SyntaxError!(ctx, "w");
	tokens
}

pub fn collect_string(ctx: &mut Context) -> String {
	let quote = ctx.current;
	ctx.next_char();
	let mut string = String::new();
	while ctx.current != quote {
		if ctx.current == '\n' {
			SyntaxError!(ctx, "unexpected new line inside of a string");
		}
		// if ctx.current == '$'
		string.push(ctx.current);
		ctx.next_char();
	}
	ctx.next_char();
	string
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
	Call(Vec<Id>, Expression)
}