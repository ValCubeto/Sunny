use std::fmt;
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
pub fn parse_static(metadata: Metadata, tokens: &mut Tokens) -> Entity {
  if metadata.is_async {
    syntax_err!("invalid async modifier");
  }
  if metadata.is_unsafe {
    syntax_err!("invalid unsafe modifier");
  }
  tokens.skip_newline();
  let ident = match tokens.next() {
    Some(Token::Ident(ident)) => ident,
    Some(_) | None => syntax_err!("expected identifier")
  };
  tokens.skip_newline();
  let Some(Token::Colon) = tokens.next() else {
    syntax_err!("expected type");
  };
  tokens.skip_newline();
  let Some(typing) = Type::parse(tokens) else {
    syntax_err!("expected type");
  };
  tokens.skip_newline();
  let Some(Token::Op(Operator::Equal)) = tokens.next() else {
    syntax_err!("expected value");
  };
  tokens.skip_newline();
  let value = parse_expr(tokens);

  match tokens.peek() {
    Some(Token::NewLine | Token::Semicolon | Token::EoF) | None => {}
    Some(other) => {
      tokens.next();
      syntax_err!("unexpected {other}, expected operator or end of statement")
    }
  }

  Entity {
    metadata,
    item: Item::Variable(Variable {
      name: ident.clone(),
      typing,
      value
    })
  }
}

#[allow(unused)]
#[derive(Debug)]
/// Any `const`, `state`, `let`, or `var`
pub struct Variable {
  pub name: String,
  pub typing: Type,
  pub value: Expr,
}

impl fmt::Display for Variable {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "var {}: {} = {}", self.name, self.typing, self.value)
  }
}
