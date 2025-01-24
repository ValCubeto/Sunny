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

  let ident = match tokens.next_token() {
    Token::Ident(ident) => ident,
    _ => syntax_err!("expected identifier")
  };

  if !matches!(tokens.next_token(), Token::Colon) {
    syntax_err!("expected type");
  };

  let typing = Typing::parse(tokens);
  if !matches!(tokens.next_token(), Token::Op(Operator::Equal)) {
    syntax_err!("expected value");
  };

  let value = Expr::parse(tokens);

  match tokens.next_token() {
    Token::Semicolon => {}
    _ => {
      tokens.next();
      syntax_err!("expected operator or end of statement");
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
    write!(f, "{}: {} = {}", self.name, self.typing, self.value)
  }
}
