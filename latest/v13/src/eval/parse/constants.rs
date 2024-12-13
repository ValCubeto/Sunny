use std::slice::Iter;
use hashbrown::HashMap;

use crate::eval::tokenize::tokens::Token;
use super::expressions::Expr;
use super::items::{Entity, Item, Metadata};
use super::types::Type;
use super::values::Value;

// Parse const and state in one function
pub fn parse_static(mutable: bool, tokens: &mut Iter<'_, Token>) -> Entity {
  // skip non-relevant tokens
  while let Some(Token::NewLine | Token::Semicolon) = tokens.next() {};
  let Some(ident) = tokens.next() else {
    syntax_err!("expected identifier");
  };
  let Some(Token::Colon) = tokens.next() else {
    syntax_err!("expected `:`");
  };
  let Some(Token::Ident(typing)) = tokens.next() else {
    syntax_err!("expected type");
  };
  let Some(Token::Equal) = tokens.next() else {
    syntax_err!("expected `=`");
  };
  // let value: parse_expr(tokens);
  todo!();
  Entity {
    metadata: Metadata::new().set_mutable(mutable as u8),
    item: Item::Const(Variable {
      name: "placeholder".to_owned(),
      typing: Type { name: "String".to_owned(), generics: HashMap::new() },
      value: Expr::Single(Value::String("value".to_owned()))
    })
  }
}

#[derive(Debug)]
/// Any const, state, let, or var
pub struct Variable {
  name: String,
  typing: Type,
  value: Expr,
}
