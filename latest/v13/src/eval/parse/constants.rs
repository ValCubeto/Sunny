use std::fmt;
use crate::eval::parse::types::parse_type;
use crate::eval::tokenize::tokens::{ Operator, Token, Tokens };
use super::expressions::{ Expr, parse_expr };
use super::items::{ Entity, Item, Metadata };
use super::types::Type;

/* Valid syntax:
const <ident>: <type> <end>
const <ident>: <type> = <expr> <end>
const { <ident>, ... }: <type> = <expr> <end>
const [ <ident>, ... ]: <type> = <expr> <end>
const ( <ident>, ... ): <type> = <expr> <end>
*/

/// Parse `const` or `state` items
pub fn parse_static(mutable: bool, tokens: &mut Tokens) -> Entity {
  // skip non-relevant tokens
  while let Some(Token::NewLine | Token::Semicolon) = tokens.peek() {
    tokens.next();
  };
  let Some(Token::Ident(ident)) = tokens.next() else {
    syntax_err!("expected identifier");
  };
  let Some(Token::Colon) = tokens.next() else {
    syntax_err!("expected typing");
  };
  let Some(typing) = parse_type(tokens) else {
    syntax_err!("expected type");
  };
  let Some(Token::Op(Operator::Equal)) = tokens.next() else {
    syntax_err!("expected equal sign");
  };
  let value = parse_expr(tokens);

  match tokens.peek() {
    Some(Token::NewLine | Token::Semicolon) | None => {}
    Some(&other) => syntax_err!("unexpected {other}")
  }

  let mut metadata = Metadata::new();
  metadata.set_mutable(mutable);
  Entity {
    metadata,
    item: Item::Const(Variable {
      name: ident.clone(),
      typing,
      value
    })
  }
}

#[allow(unused)]
#[derive(Debug)]
/// **Any `const`, `state`, `let`, or `var`**
pub struct Variable {
  name: String,
  typing: Type,
  value: Expr,
}

impl fmt::Display for Variable {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "var {}: {:?} = {}", self.name, self.typing, self.value)
  }
}
