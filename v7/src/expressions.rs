use crate::{context::Context, values::Value, errors::SyntaxError, id::Id};

pub fn parse_expr(ctx: &mut Context) -> Vec<Expression> {
	let mut Expressions = Vec::new();
	ctx.go();
	match ctx.current {
		'"' | '\'' => {
			let string = collect_string(ctx);
			ctx.skip_spaces();

			match ctx.current {
				'\n' => Expressions.push(Expression::Value(Value::String(string))),
				_ => SyntaxError!(ctx, "unexpected character {:?}", ctx.current)
			}
		}
		_ => SyntaxError!(ctx, "unexpected character {:?}", ctx.current)
	}
	// order Expressions
	// SyntaxError!(ctx, "w");
	Expressions
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
pub enum Token {
	Expression(Box<Expression>),
	Value(Value)
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Expression {
	Value(Value),
	Sum(Token, Token),
	Sub(Token, Token),
	Mul(Token, Token),
	Div(Token, Token),
	Call(Vec<Id>, Token)
}