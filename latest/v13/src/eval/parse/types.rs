use std::fmt::{ self, Write };
use hashbrown::HashMap;
use crate::eval::tokenize::{
  keywords::Keyword,
  tokens::{ Operator, Token, Tokens },
};
use crate::eval::parse::functions::Function;

#[allow(unused)]
#[derive(Debug, Clone)]
/// # Valid forms
/// (undefined)
/// T
/// T<A: (T) = (T), B: (T) = (T)>
/// (T) | (T)
/// (T) + (T)
/// (T) for (T)
/// fun \((T)\): (T)
pub enum Typing {
  Undefined,
  Single(Type),
  Tuple(Vec<Type>),
  Or(Vec<Type>),
  And(Vec<Type>),
  Impl(Type, Type),
  Fun {
    args: Vec<Typing>,
    output: Box<Typing>
  }
}

impl Typing {
  pub fn parse(tokens: &mut Tokens) -> Typing {
    // Skip leading `|`
    if matches!(tokens.peek(), Token::Op(Operator::Pipe)) {
      tokens.next();
    }
    let name = match tokens.next() {
      Token::Ident(ident) => ident.clone(),
      Token::Keyword(Keyword::Fun) => syntax_err!("functions as types not yet implemented"),
      Token::Op(Operator::LeftAngle) => syntax_err!("`<T>` syntax not yet implemented"),
      Token::Keyword(Keyword::Impl) => syntax_err!("`impl T` syntax not yet implemented"),
      other => syntax_err!("unexpected {other}")
    };
    let generics = HashMap::new();
    if let Token::Op(Operator::LeftAngle) = tokens.peek() {
      tokens.next();
      syntax_err!("`{name}<...>` syntax not yet implemented");
    }
    if let Token::Keyword(Keyword::For) = tokens.peek() {
      tokens.next();
      syntax_err!("`{name} for T` syntax not yet implemented");
    }
    if let Token::Op(Operator::Pipe) = tokens.peek() {
      tokens.next();
      syntax_err!("`{name} | T` syntax not yet implemented");
    }
    if let Token::Op(Operator::Plus) = tokens.peek() {
      tokens.next();
      syntax_err!("`{name} + T` syntax not yet implemented");
    }
    Typing::Single(Type { name, generics })
  }
  pub fn from_function(function: &Function) -> Typing {
    Typing::Fun {
      args: function.params.iter().map(|p| p.typing.clone()).collect(),
      output: Box::new(function.output.clone())
    }
  }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Type {
  pub name: String,
  pub generics: HashMap<String, Type>,
}

impl fmt::Display for Type {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.name)?;
    if !self.generics.is_empty() {
      f.write_char('<')?;
      let mut generics = self.generics.iter();
      // Safety: generics is not empty
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
