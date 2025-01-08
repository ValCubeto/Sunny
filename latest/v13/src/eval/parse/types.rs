use hashbrown::HashMap;
use crate::eval::tokenize::tokens::{ Operator, Token, Tokens };

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
    Token::Op(Operator::LeftAngle) => todo!(),
    _ => return None,
  };
  let generics = HashMap::new();
  Some(Type { name, generics })
}
