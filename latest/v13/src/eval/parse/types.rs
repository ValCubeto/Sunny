use std::fmt::{self, Write};

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

impl fmt::Display for Type {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.name)?;
    if !self.generics.is_empty() {
      f.write_char('<')?;
      let mut generics = self.generics.iter();
      let (key, value) = generics.next().unwrap();
      if key.chars().map(|ch| ch.is_ascii_digit()).all(|p| p) {
        write!(f, "{value}")?;
      } else {
        write!(f, "{key}: {value}")?;
      }
      for (key, value) in generics {
        // it will stop as soon as it finds a non-digit character
        if key.chars().map(|ch| ch.is_ascii_digit()).all(|p| p) {
          write!(f, ", {value}")?;
        } else {
          write!(f, ", {key}: {value}")?;
        }
      }
      f.write_char('>')?;
    }
    Ok(())
  }
}

impl Type {
  pub fn parse(tokens: &mut Tokens) -> Option<Type> {
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
}
