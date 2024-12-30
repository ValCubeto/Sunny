use crate::eval::parse::types::parse_type;
use crate::eval::tokenize::tokens::{ Token, Tokens };
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

// Parse const and state in one function
pub fn parse_static(mutable: bool, tokens: &mut Tokens) -> Entity {
  // skip non-relevant tokens
  while let Some(Token::NewLine | Token::Semicolon) = tokens.peek() {
    tokens.next();
  };
  let Some(Token::Ident(ident)) = tokens.next() else {
    syntax_err!("expected identifier");
  };
  let Some(Token::Colon) = tokens.next() else {
    syntax_err!("expected `:`");
  };
  let Some(typing) = parse_type(tokens) else {
    syntax_err!("expected type");
  };
  let Some(Token::Equal) = tokens.next() else {
    syntax_err!("expected `=`");
  };
  let Some(value) = parse_expr(tokens) else {
    syntax_err!("expected expression");
  };
  Entity {
    metadata: Metadata::new().set_mutable(mutable as u8),
    item: Item::Const(Variable {
      name: ident.clone(),
      typing,
      value
    })
  }
}

#[allow(unused)]
#[derive(Debug)]
/// Any const, state, let, or var
pub struct Variable {
  name: String,
  typing: Type,
  value: Expr,
}
