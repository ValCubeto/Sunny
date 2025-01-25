use std::fmt;

use crate::eval::parse::expressions::Expr;
use crate::eval::tokenize::{
  keywords::Keyword as Kw,
  tokens::{ Operator as Op, Token as Tk, TokenIter }
};

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Statement {
  /// No statements
  None,
  Expr(Expr),
  /// `a = b`
  Assign(Expr, Expr),
  /// `if a { b } else { c }`
  If {
    cond: Expr,
    yes: Block,
    no: Block
  },
  /// `return a`
  Return(Expr)
}

impl Statement {
  pub fn parse_block(tokens: &mut TokenIter) -> Block {
    let mut body = Vec::new();
    loop {
      match Self::parse_single(tokens) {
        Statement::None => {}
        stmt => body.push(stmt)
      }
      if !tokens.semicolon_sep() {
        break;
      }
    }
    Block(body)
  }
  pub fn parse_single(tokens: &mut TokenIter) -> Statement {
    match tokens.peek_token() {
      Tk::RightBrace | Tk::RightBracket | Tk::RightParen => Statement::None,
      Tk::Keyword(Kw::Let) => {
        tokens.next();
        syntax_err!("let statements not yet implemented");
        // body.push(Expr::parse(tokens));
      }
      Tk::Keyword(Kw::Var) => {
        tokens.next();
        syntax_err!("var statements not yet implemented");
      }
      Tk::Keyword(Kw::If) => {
        tokens.next();
        syntax_err!("if statements not yet implemented");
      }
      Tk::Keyword(Kw::Loop) => {
        tokens.next();
        syntax_err!("loops not yet implemented");
      }
      Tk::Keyword(Kw::While) => {
        tokens.next();
        syntax_err!("loops not yet implemented");
      }
      Tk::Keyword(Kw::For) => {
        tokens.next();
        syntax_err!("for loops not yet implemented");
      }
      Tk::Keyword(Kw::Match) => {
        tokens.next();
        syntax_err!("match statements not yet implemented");
      }
      Tk::Keyword(Kw::Defer) => {
        tokens.next();
        syntax_err!("defer blocks not yet implemented");
      }
      Tk::Keyword(Kw::Return) => {
        tokens.next();
        syntax_err!("returns not yet implemented");
      }
      _ => {
        match Expr::parse(tokens) {
          Expr::None => Statement::None,
          expr => {
            match tokens.peek_token() {
              Tk::Op(Op::Equal) => {
                tokens.next();
                Statement::Assign(expr, Expr::parse(tokens))
              }
              other => {
                if tokens.semicolon_sep() {
                  Statement::Expr(expr)
                } else {
                  tokens.next();
                  note!("it may be a bug while parsing the expression");
                  syntax_err!("unexpected {other}");
                }
              }
            }
          }
        } // match expr
      }
    }
  }
}

impl fmt::Display for Statement {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::None => Ok(()),
      Self::Expr(expr) => write!(f, "{expr}"),
      Self::Assign(lhs, rhs) => write!(f, "{lhs} = {rhs}"),
      Self::If { cond, yes, no } => write!(f, "if {cond} {yes} else {no}"),
      Self::Return(expr) => write!(f, "return {expr}"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Block(pub Vec<Statement>);
impl fmt::Display for Block {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{{ ")?;
    for stmt in self.0.iter() {
      write!(f, "{stmt}; ")?;
    }
    write!(f, "}}")
  }
}
