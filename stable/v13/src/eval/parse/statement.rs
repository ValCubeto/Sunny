use crate::eval::parse::expressions::Expr;
use crate::eval::tokenize::{
  keywords::Keyword as Kw,
  tokens::{ Operator as Op, Token as Tk, Tokens }
};

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Statement {
  Expr(Expr),
  Assign(Expr, Expr),
  If {
    cond: Expr,
    yes: Vec<Statement>,
    no: Vec<Statement>
  },
  Return(Expr)
}

impl Statement {
  pub fn parse(tokens: &mut Tokens) -> Vec<Statement> {
    let mut body = Vec::new();
    match tokens.peek_token() {
      Tk::RightBrace => {},
      Tk::Keyword(Kw::Let) => {
        syntax_err!("let statements not yet implemented");
        // body.push(Expr::parse(tokens));
      }
      Tk::Keyword(Kw::Var) => {
        syntax_err!("var statements not yet implemented");
      }
      Tk::Keyword(Kw::If) => {
        syntax_err!("if statements not yet implemented");
      }
      Tk::Keyword(Kw::Loop) => {
        syntax_err!("loops not yet implemented");
      }
      Tk::Keyword(Kw::While) => {
        syntax_err!("loops not yet implemented");
      }
      Tk::Keyword(Kw::For) => {
        syntax_err!("for loops not yet implemented");
      }
      Tk::Keyword(Kw::Match) => {
        syntax_err!("match statements not yet implemented");
      }
      Tk::Keyword(Kw::Defer) => {
        syntax_err!("defer blocks not yet implemented");
      }
      Tk::Keyword(Kw::Return) => {
        syntax_err!("returns not yet implemented");
      }
      _ => {
        let expr = Expr::parse(tokens);
        match tokens.peek_token() {
          Tk::Op(Op::Equal) => {
            tokens.next();
            let stmt = Statement::Assign(expr, Expr::parse(tokens));
            body.push(stmt);
          }
          _ => {
            body.push(Statement::Expr(expr));
          }
        }
      }
    }
    body
  }
}
