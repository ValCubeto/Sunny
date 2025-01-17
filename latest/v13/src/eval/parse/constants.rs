use std::fmt;
use crate::eval::tokenize::tokens::{ Operator, Token, Tokens };
use crate::eval::parse::{
  expressions::Expr,
  items::{ Entity, Item, Metadata },
  types::Typing
};

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
    Token::Ident(ident) => ident,
    _ => syntax_err!("expected identifier")
  };
  tokens.skip_newline();
  if !matches!(tokens.next(), Token::Colon) {
    syntax_err!("expected type");
  };
  tokens.skip_newline();
  let typing = Typing::parse(tokens);
  tokens.skip_newline();
  if !matches!(tokens.next(), Token::Op(Operator::Equal)) {
    syntax_err!("expected value");
  };
  tokens.skip_newline();
  let value = Expr::parse(tokens);

  match tokens.peek() {
    Token::NewLine | Token::Semicolon | Token::EoF => {}
    other => {
      tokens.next();
      syntax_err!("expected operator or end of statement, found {other}");
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
  pub typing: Typing,
  pub value: Expr,
}

impl fmt::Display for Variable {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "var {}: {:?} = {}", self.name, self.typing, self.value)
  }
}
