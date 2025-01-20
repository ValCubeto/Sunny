use std::fmt;
use crate::eval::parse::{
  types::join,
  constants::Variable,
  expressions::Expr,
  items::{ Entity, Item, Metadata },
  types::Typing,
  values::Value,
  statement::Statement
};
use crate::eval::tokenize::{
  keywords::Keyword as Kw,
  tokens::{ Operator as Op, Token as Tk, Tokens }
};

use super::types::GenericParam;

// #region params
pub fn display_param(name: &str, typing: &Typing, default_val: &dyn fmt::Display) -> String {
  let mut string = name.to_owned();
  if !matches!(typing, Typing::Undefined) {
    string.push_str(": ");
    string.push_str(&typing.to_string());
  }
  let default_val = default_val.to_string();
  if !default_val.is_empty() {
    string.push_str(" = ");
    string.push_str(&default_val);
  }
  string
}

#[allow(unused)]
#[derive(Debug)]
pub struct Param {
  pub name: String,
  pub typing: Typing,
  pub default_val: Expr,
}

impl fmt::Display for Param {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", display_param(&self.name, &self.typing, &self.default_val))
  }
}
// #endregion params

#[allow(unused)]
#[derive(Debug)]
pub struct Function {
  pub name: String,
  pub params: Vec<Param>,
  pub generics: Vec<GenericParam>,
  pub output: Typing,
  pub body: Vec<Statement>,
}

impl Function {
// Some functions have no name
  pub fn parse(mut metadata: Metadata, tokens: &mut Tokens, name: String) -> Entity {
    metadata.mutable = false;
    if metadata.is_async && metadata.is_static {
      syntax_err!("a function cannot be both async and static");
    }
    if metadata.is_async {
      syntax_err!("async functions not yet implemented");
    }
    if metadata.is_unsafe {
      syntax_err!("unsafe functions not yet implemented");
    }
    if metadata.is_static {
      syntax_err!("static functions not yet implemented");
    }

    let generics = parse_generics(tokens);
    let mut params = Vec::new();
    match tokens.next_token() {
      Tk::LeftParen => {
        #[allow(clippy::never_loop)]
        loop {
          match tokens.peek_token() {
            Tk::NewLine => continue,
            Tk::RightParen => break,
            Tk::Ident(name) => {
              tokens.next();
              let typing = match tokens.peek_token() {
                Tk::Colon => {
                  tokens.next();
                  Typing::parse(tokens)
                }
                _ => Typing::Undefined
              };
              let default_val = match tokens.peek_token() {
                Tk::Op(Op::Equal) => {
                  tokens.next();
                  Expr::parse(tokens)
                }
                _ => Expr::None
              };
              params.push(Param {
                name: name.clone(),
                typing,
                default_val
              });
              if let Tk::Comma | Tk::NewLine = tokens.peek() {
                tokens.next();
              } else {
                break;
              }
            }
            Tk::LeftBrace => {
              syntax_err!("parameter destructuring not yet implemented");
            }
            Tk::LeftParen => {
              syntax_err!("parameter destructuring not yet implemented");
            }
            Tk::LeftBracket => {
              syntax_err!("parameter destructuring not yet implemented");
            }
            _ => syntax_err!("expected parameter list")
          }
        }
      }
      _ => syntax_err!("expected parameters")
    }
    if !matches!(tokens.next_token(), Tk::RightParen) {
      syntax_err!("unclosed parenthesis");
    }

    let output = match tokens.peek_token() {
      Tk::Arrow => {
        tokens.next();
        Typing::parse(tokens)
      }
      _ => Typing::Undefined
    };

    if matches!(tokens.peek_token(), Tk::Keyword(Kw::Takes)) {
      tokens.next();
      syntax_err!("self takes not yet implemented");
    }

    let mut body = Vec::new();
    if matches!(tokens.next_token(), Tk::LeftBrace) {
      body = Statement::parse(tokens);
      match tokens.next_token() {
        Tk::NewLine | Tk::Semicolon => {}
        other => syntax_err!("unexpected {other}")
      }
    }
    let function = Function {
      name: name.clone(),
      params,
      generics,
      output,
      body
    };
    Entity {
      metadata,
      item: Item::Variable(Variable {
        name,
        typing: Typing::from_function(&function),
        value: Expr::Single(Value::Function(function))
      })
    }
  }
}

impl fmt::Display for Function {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "fun {}", self.name)?;
    if !self.generics.is_empty() {
      write!(f, "<{}>", join(self.generics.iter(), ", "))?;
    }
    write!(f, "({})", join(self.params.iter(), ", "))?;
    if !matches!(self.output, Typing::Undefined) {
      write!(f, " -> {}", self.output)?;
    }
    Ok(())
  }
}
