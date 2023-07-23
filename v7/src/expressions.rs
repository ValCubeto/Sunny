use crate::{context::Context, values::Value, errors::SyntaxError, id::Id};

pub fn parse_expr(ctx: &mut Context) -> Expression {
	let mut expr = Expression::Value(Value::None);
	ctx.go();
	// parse until unexpected char
	expr
}

pub fn parse_body(ctx: &mut Context) -> Vec<Expression> {
	let mut expressions = Vec::new();
	ctx.go();
	match ctx.current {
		'"' | '\'' => {
			let string = collect_string(ctx);
			ctx.skip_spaces();

			if ctx.is_valid_id() {
				SyntaxError!(ctx, "to-do")
			}
			match ctx.current {
				';' => expressions.push(Expression::Value(Value::String(string))),
				'\n' => {
					ctx.go();
					SyntaxError!(ctx, "usa el punto y coma flaco");
				}
				_ => SyntaxError!(ctx, "unexpected character {:?}", ctx.current)
			}
		}
		_ => SyntaxError!(ctx, "unexpected character {:?}", ctx.current)
	}
	// order Expressions
	// SyntaxError!(ctx, "w");
	dbg!(&expressions);
	expressions
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