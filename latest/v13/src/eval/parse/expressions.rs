use std::slice::Iter;
use crate::eval::tokenize::tokens::Token;
use super::values::Value;

pub fn parse_expr(tokens: &mut Iter<'_, Token>) -> Expr {
  todo!();
}

#[derive(Debug)]
pub enum Expr {
  Single(Value),
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Pow,

  If,
  For,

  Let,
  Var,
}
