use hashbrown::HashMap;
use crate::eval::tokenize::{
  keywords::Keyword,
  tokens::{ Operator, Token, Tokens }
};

// T<A, B: C>
#[allow(unused)]
#[derive(Debug)]
pub struct Type {
  pub name: String,
  // /// `I for T`
  // pub impl_for: String,
  pub generics: HashMap<String, Type>,
}

pub fn parse_type(tokens: &mut Tokens) -> Option<Type> {
  let name = match tokens.next()? {
    Token::Ident(ident) => ident.clone(),
    Token::Op(Operator::LeftAngle) => syntax_err!("`<T>` syntax not yet implemented"),
    Token::Keyword(Keyword::Impl) => syntax_err!("`impl T` syntax not yet implemented"),
    other => syntax_err!("unexpected {other}, expected type"),
  };
  let generics = HashMap::new();
  if let Token::Op(Operator::LeftAngle) = tokens.peek()? {
    tokens.next();
    syntax_err!("`{name}<...>` syntax not yet implemented");
  }
  if let Token::Keyword(Keyword::For) = tokens.peek()? {
    tokens.next();
    syntax_err!("`{name} for T` syntax not yet implemented");
  }
  Some(Type { name, generics })
}
