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

type E = Box<Expr>;

#[allow(unused)]
#[derive(Debug)]
pub enum Expr {
  Single(Value),
  Add(E, E),
  Sub(E, E),
  Mul(E, E),
  Div(E, E),
  Mod(E, E),
  Pow(E, E),

  // If(E, E),
  // For(E, E),

  // Let(E, E),
  // Var(E, E),
}

impl Expr {
  pub fn ptr(self) -> Box<Expr> {
    Box::new(self)
  }
}
