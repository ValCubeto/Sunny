use crate::{context::Context, values::Value, SyntaxError, id::Id};

pub fn parse_expr(ctx: &mut Context) -> Expression {
	ctx.go();
	match ctx.current {
		'"' | '\'' => {
			return Expression::Value(Value::String(collect_string(ctx)))
		}
		_ => SyntaxError!(ctx, "to-do {:?}", ctx.current)
	}
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
	Value(Value),
	Sum(Box<Expression>, Box<Expression>),
	Sub(Box<Expression>, Box<Expression>),
	Mul(Box<Expression>, Box<Expression>),
	Div(Box<Expression>, Box<Expression>),
	Call(Vec<Id>, Box<Expression>)
}