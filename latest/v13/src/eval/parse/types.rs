use hashbrown::HashMap;
use crate::eval::tokenize::tokens::{ Token, Tokens };

// TODO: `I for T`, `<T>`

// T<A, B: C>
#[allow(unused)]
#[derive(Debug)]
pub struct Type {
  pub name: String,
  pub generics: HashMap<String, Type>,
}

pub fn parse_type(tokens: &mut Tokens) -> Option<Type> {
  let generics = HashMap::new();
  let name = match tokens.next()? {
    Token::Ident(ident) => ident.clone(),
    Token::LeftAngle => todo!(),
    _ => syntax_err!("unexpected token"),
  };
  Some(Type { name, generics })
}
