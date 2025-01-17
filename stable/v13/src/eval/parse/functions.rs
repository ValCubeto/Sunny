use std::fmt;
use crate::eval::{parse::types::join, tokenize::{
  keywords::Keyword,
  tokens::{ Operator, Token, Tokens }
}};
use crate::eval::parse::{
  constants::Variable,
  expressions::Expr,
  items::{ Entity, Item, Metadata },
  types::Typing,
  values::Value,
  statement::Statement
};

fn display_param(name: &str, typing: &Typing, default_val: &dyn fmt::Display) -> String {
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
pub struct GenericParam {
  pub name: String,
  pub typing: Typing,
  pub default_val: Typing,
}

impl fmt::Display for GenericParam {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", display_param(&self.name, &self.typing, &self.default_val))
  }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Param {
  pub name: String,
  pub typing: Typing,
  // Default value
  pub default_val: Expr,
}

impl fmt::Display for Param {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", display_param(&self.name, &self.typing, &self.default_val))
  }
}

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
    if metadata.is_async {
      syntax_err!("async functions not yet implemented");
    }
    if metadata.is_unsafe {
      syntax_err!("unsafe functions not yet implemented");
    }
    if metadata.is_const {
      syntax_err!("const functions not yet implemented");
    }

    let mut generics: Vec<GenericParam> = Vec::new();
    tokens.skip_newline();
    match tokens.peek() {
      Token::Op(Operator::Diamond) => {
        // empty generics
        tokens.next();
      }
      Token::Op(Operator::LeftAngle) => {
        tokens.next();
        loop {
          tokens.skip_newline();
          match tokens.next() {
            Token::Op(Operator::RightAngle) => break,
            Token::Ident(ident) => {
              let typing = match tokens.peek() {
                Token::Colon => {
                  tokens.next();
                  Typing::parse(tokens)
                }
                _ => Typing::Undefined
              };
              let default_val = match tokens.peek() {
                Token::Op(Operator::Equal) => {
                  tokens.next();
                  Typing::parse(tokens)
                }
                // _ => Typing::Undefined
                other => {
                  debug!(other);
                  Typing::Undefined
                }
              };
              generics.push(GenericParam {
                name: ident.clone(),
                typing,
                default_val
              });
            }
            _ => break
          }
        }
      }
      _ => {}
    }
    let mut params = Vec::new();
    tokens.skip_newline();
    match tokens.next() {
      Token::LeftParen => {
        #[allow(clippy::never_loop)]
        loop {
          match tokens.peek() {
            Token::NewLine => continue,
            Token::RightParen => break,
            Token::Ident(_name) => {
              syntax_err!("function parameters not yet implemented");
            }
            Token::LeftBrace => {
              syntax_err!("parameter destructuring not yet implemented");
            }
            Token::LeftParen => {
              syntax_err!("parameter destructuring not yet implemented");
            }
            Token::LeftBracket => {
              syntax_err!("parameter destructuring not yet implemented");
            }
            _ => syntax_err!("expected parameter list")
          }
        }
      }
      _ => syntax_err!("expected parameters")
    }
    if !matches!(tokens.next(), Token::RightParen) {
      syntax_err!("unclosed parenthesis");
    }

    tokens.skip_newline();
    let output = match tokens.peek() {
      Token::Arrow => {
        tokens.next();
        Typing::parse(tokens)
      }
      _ => Typing::Undefined
    };

    tokens.skip_newline();
    if matches!(tokens.peek(), Token::Keyword(Keyword::Takes)) {
      tokens.next();
      syntax_err!("self takes not yet implemented");
      // tokens.skip_newline();
    }

    let mut body = Vec::new();
    if matches!(tokens.next(), Token::LeftBrace) {
      body = Statement::parse(tokens);
      let token = tokens.next();
      if !matches!(token, Token::RightBrace) {
        syntax_err!("unexpected {token}");
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
