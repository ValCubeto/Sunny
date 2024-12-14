use crate::eval::tokenize::tokens::{ Token, Tokens };
use super::values::Value;

pub fn parse_expr(tokens: &mut Tokens) -> Option<Expr> {
  let token = tokens.next()?;
  parse_expr_(expect_value(token), tokens)
}

fn parse_expr_(prev: Expr, tokens: &mut Tokens) -> Option<Expr> {
  match tokens.next()? {
    Token::Plus => parse_expr_(Expr::Add(prev.ptr(), expect_value(tokens.next()?).ptr()), tokens),
    _ => Some(prev),
  }
}

fn expect_value(value: &Token) -> Expr {
  Expr::Single(match value {
    Token::String(string) => Value::String(string.clone()),
    // Token::Int(_) => ,
    // Token::Float(_) => ,
    _ => syntax_err!("expected value"),
  })
}

#[derive(Debug)]
pub enum Expr {
  Single(Value),
  Add(Box<Expr>, Box<Expr>),
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

impl Expr {
  pub fn ptr(self) -> Box<Expr> {
    Box::new(self)
  }
}
