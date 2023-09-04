use crate::{
  context::Context,
  values::Value,
  aliases::Id,
  syntax_error,
};

pub fn parse_expr(ctx: &mut Context) -> Expression {
  ctx.go();
  if ctx.current.is_alphabetic() {
    ctx.debug();
    let word = ctx.collect_word();
    syntax_error!("id: {word:?}"; ctx);
  }
  match ctx.current {
    '"' | '\'' => {
      // return
      Expression::Value(Value::String(collect_string(ctx)))
    }
    _ => syntax_error!("unexpected character {:?}", ctx.current; ctx)
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
    // if ctx.current == '$' { ctx.next_char(); if ctx.current != '(' { idk } }
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
	Pow(Box<Expression>, Box<Expression>),
	// Mod, In

	Eq(Box<Expression>, Box<Expression>),
	Neq(Box<Expression>, Box<Expression>),
	// Gt, Lt, Geq, Leq

	// LogAnd, LogOr, LogXor
	And(Box<Expression>, Box<Expression>),
	Or(Box<Expression>, Box<Expression>),
}