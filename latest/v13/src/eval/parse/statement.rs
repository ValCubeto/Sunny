use crate::eval::parse::{
  expressions::Expr,
  values::Value
};
use crate::eval::tokenize::{
  keywords::Keyword,
  tokens::{ Operator, Token, Tokens }
};

// type S = Box<Statement>;

#[allow(unused)]
pub enum Statement {
  Expr(Expr),
  Assign(Expr, Expr),
  If {
    cond: Expr,
    yes: Vec<Statement>,
    no: Vec<Statement>,
  },
}

impl Statement {
  pub fn parse(tokens: &mut Tokens) -> Vec<Expr> {
    let mut body = Vec::new();
    while let Some(token) = tokens.try_next() {
      match token {
        Token::NewLine | Token::Semicolon => continue,
        Token::RightBrace => break,
        Token::Ident(name) => {
          let mut expr = Expr::Single(Value::Ident(name.clone()));
          while let Some(token) = tokens.try_next() {
            match token {
              Token::NewLine | Token::Semicolon => break,
              Token::Op(Operator::Equal) => {
                tokens.next();
                expr = Expr::Single(Value::Ident(name.clone()));
              }
              Token::RightBrace => break,
              _ => syntax_err!("unexpected {token}")
            }
          }
          body.push(expr);
        }
        Token::EoF => syntax_err!("unexpected end of file"),
        Token::Keyword(Keyword::Let) => {
          syntax_err!("let statements not yet implemented");
          // body.push(Expr::parse(tokens));
        }
        Token::Keyword(Keyword::Var) => {
          syntax_err!("var statements not yet implemented");
        }
        Token::Keyword(Keyword::If) => {
          syntax_err!("if statements not yet implemented");
        }
        Token::Keyword(Keyword::Loop) => {
          syntax_err!("loops not yet implemented");
        }
        Token::Keyword(Keyword::While) => {
          syntax_err!("loops not yet implemented");
        }
        Token::Keyword(Keyword::For) => {
          syntax_err!("for loops not yet implemented");
        }
        Token::Keyword(Keyword::Match) => {
          syntax_err!("match statements not yet implemented");
        }
        Token::Keyword(Keyword::Defer) => {
          syntax_err!("defer blocks not yet implemented");
        }
        Token::Keyword(Keyword::Return) => {
          syntax_err!("returns not yet implemented");
        }
        _ => break
      }
    }
    body
  }
}
