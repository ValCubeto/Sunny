use crate::eval::parse::expressions::Expr;
use crate::eval::tokenize::{
  keywords::Keyword,
  tokens::{ Operator, Token, Tokens }
};

#[allow(unused)]
#[derive(Debug)]
pub enum Statement {
  Expr(Expr),
  Return(Expr),
  Assign(Expr, Expr),
  If {
    cond: Expr,
    yes: Vec<Statement>,
    no: Vec<Statement>
  }
}

impl Statement {
  pub fn parse(tokens: &mut Tokens) -> Vec<Statement> {
    let mut body = Vec::new();
    tokens.skip_separator();
    while let Some(token) = tokens.try_peek() {
      match token {
        Token::EoF => syntax_err!("unexpected end of file"),
        Token::RightBrace => break,
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
        _ => {
          let expr = Expr::parse(tokens);
          while let Some(token) = tokens.try_peek() {
            match token {
              Token::NewLine | Token::Semicolon => {
                tokens.next();
              }
              Token::Op(Operator::Equal) => {
                tokens.next();
                let stmt = Statement::Assign(expr, Expr::parse(tokens));
                body.push(stmt);
                break;
              }
              _ => {
                body.push(Statement::Expr(expr));
                break;
              } // _
            } // match
          } // while
        } // _
      } // match
      tokens.skip_separator();
    } // while
    debug!(tokens.peek());
    debug_todo!("return from last statement");
    body
  }
}
