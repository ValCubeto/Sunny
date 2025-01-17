use std::fmt::{ self, Write };
use hashbrown::HashMap;
use crate::eval::tokenize::{
  keywords::Keyword,
  tokens::{ Operator, Token, Tokens },
};
use crate::eval::parse::functions::Function;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Type {
  /// `path::to::Type`
  pub name: Vec<String>,
  pub generics: HashMap<String, Typing>,
}

impl fmt::Display for Type {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.name.join("::"))?;
    if !self.generics.is_empty() {
      f.write_char('<')?;
      let mut generics = self.generics.iter();
      // Safety: generics is not empty
      let (key, value) = generics.next().unwrap();
      if key.chars().all(|ch| ch.is_ascii_digit()) {
        write!(f, "{value}")?;
      } else {
        write!(f, "{key}: {value}")?;
      }
      for (key, value) in generics {
        // it will stop as soon as it finds a non-digit character
        if key.chars().all(|ch| ch.is_ascii_digit()) {
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

#[allow(unused)]
#[derive(Debug, Clone)]
/// # Valid forms
/// (undefined)
/// T
/// T<A: T = T, B: T = T>
/// T | T
/// T + T
/// T for T
/// fun (T) -> T | (T) => T
pub enum Typing {
  Undefined,
  Single(Type),
  Tuple(Vec<Typing>),
  Or(Vec<Typing>),
  And(Vec<Typing>),
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
    let mut name = Vec::with_capacity(1);
    loop {
      match tokens.peek() {
        Token::Ident(ident) => {
          tokens.next();
          name.push(ident.clone());
          while let Token::Op(Operator::DoubleColon) = tokens.peek() {
            tokens.next();
            match tokens.next() {
              Token::Ident(ident) => name.push(ident.clone()),
              _ => syntax_err!("expected type name")
            }
          }
        }
        Token::Keyword(Keyword::Fun) => {
          tokens.next();
          syntax_err!("functions as types not yet implemented");
        }
        Token::LeftParen => {
          tokens.next();
          syntax_err!("functions as types not yet implemented");
        }
        Token::Op(Operator::LeftAngle) => {
          tokens.next();
          syntax_err!("`<T>` syntax not yet implemented");
        }
        Token::Keyword(Keyword::Impl) => {
          tokens.next();
          syntax_err!("`impl T` syntax not yet implemented");
        }
        _ => break
      };
    }
    let mut generics = HashMap::new();
    if let Token::Op(Operator::LeftAngle) = tokens.peek() {
      tokens.next();
      syntax_err!("`{}<...>` syntax not yet implemented", name.join("::"));
    }
    if let Token::Keyword(Keyword::For) = tokens.peek() {
      tokens.next();
      syntax_err!("`{} for T` syntax not yet implemented", name.join("::"));
    }
    if let Token::Op(Operator::Pipe) = tokens.peek() {
      tokens.next();
      syntax_err!("`{} | T` syntax not yet implemented", name.join("::"));
    }
    if let Token::Op(Operator::Plus) = tokens.peek() {
      tokens.next();
      syntax_err!("`{} + T` syntax not yet implemented", name.join("::"));
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

/// Joins a list of displayable items
pub fn join<T: fmt::Display>(iter: impl Iterator<Item = T>, sep: &str) -> String {
  iter.map(|ty| ty.to_string()).collect::<Vec<_>>().join(sep)
}

impl fmt::Display for Typing {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Undefined => Ok(()),
      Self::Single(ty) => write!(f, "{ty}"),
      Self::Tuple(tys) => write!(f, "({})", join(tys.iter(), ", ")),
      Self::Or(tys) => write!(f, "{}", join(tys.iter(), " | ")),
      Self::And(tys) => write!(f, "{}", join(tys.iter(), " + ")),
      Self::Impl(ty, ty2) => write!(f, "{ty} for {ty2}"),
      Self::Fun { args, output } => write!(f, "fun ({}) -> {output}", join(args.iter(), ", "))
    }
  }
}
