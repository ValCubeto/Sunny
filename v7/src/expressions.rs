use crate::{context::Context, values::Value, syntax_error, id::Id};

pub fn parse_expr(ctx: &mut Context) -> Expression {
	ctx.go();
	if ctx.is_valid_id() {
		let word = ctx.collect_word();
		syntax_error!("id: {word:?}"; ctx);
	}
	match ctx.current {
		'"' | '\'' => {
			// return
			Expression::Value(Value::String(collect_string(ctx)))
		}
		_ => syntax_error!("to-do {:?}", ctx.current; ctx)
	}
}

pub fn collect_string(ctx: &mut Context) -> String {
	let quote = ctx.current;
	ctx.next_char();
	let mut string = String::new();
	while ctx.current != quote {
		if ctx.current == '\n' {
			syntax_error!("unexpected new line inside of a string"; ctx);
		}
		// if ctx.current == '$'
		string.push(ctx.current);
		ctx.next_char();
	}
	ctx.next_char();
	string
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
	Value(Value),
	Sum(Box<Expression>, Box<Expression>),
	Sub(Box<Expression>, Box<Expression>),
	Mul(Box<Expression>, Box<Expression>),
	Div(Box<Expression>, Box<Expression>),
	Call(Vec<Id>, Box<Expression>)
}