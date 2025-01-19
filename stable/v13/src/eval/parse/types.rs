use std::fmt;
use crate::eval::{
  tokenize::{
    keywords::Keyword as Kw,
    tokens::{ Operator as Op, Token as Tk, Tokens },
  },
  parse::functions::{ Function, display_param }
};

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct GenericParam {
  pub name: String,
  pub typing: Typing,
  pub default_val: Typing,
}

impl fmt::Display for GenericParam {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", display_param(&self.name, &self.typing, &self.default_val))
  }
}

pub fn parse_generics(tokens: &mut Tokens) -> Vec<GenericParam> {
  let mut generics = Vec::new();
  match tokens.peek_token() {
    // empty generics
    Tk::Op(Op::Diamond) => {
      tokens.next();
    }
    Tk::Op(Op::LeftAngle) => {
      tokens.next();
      while let Tk::Ident(name) = tokens.peek_token() {
        tokens.next();
        let typing = match tokens.peek_token() {
          Tk::Colon => {
            tokens.next();
            Typing::parse(tokens)
          }
          _ => Typing::Undefined
        };
        let default_val = match tokens.peek_token() {
          Tk::Op(Op::Equal) => {
            tokens.next();
            Typing::parse(tokens)
          }
          _ => Typing::Undefined
        };
        generics.push(GenericParam {
          name: name.clone(),
          typing,
          default_val
        });
        if let Tk::Comma | Tk::NewLine = tokens.peek() {
          tokens.next();
        } else {
          break;
        }
      }
    }
    _ => {}
  }
  match tokens.next_token() {
    Tk::Op(Op::RightAngle) => generics,
    other => syntax_err!("unexpected {other}")
  }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Type {
  /// `path::to::Type`
  pub name: Vec<String>,
  pub generics: Vec<GenericParam>,
}

impl fmt::Display for Type {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.name.join("::"))?;
    if !self.generics.is_empty() {
      let generics = self.generics.iter().map(|(key, value)| {
        if key.chars().all(|ch| ch.is_ascii_digit()) {
          format!("{value}")
        } else {
          format!("{key}: {value}")
        }
      });
      write!(f, "<{}>", generics.collect::<Vec<_>>().join(", "))?;
    }
    Ok(())
  }
}

#[allow(unused)]
#[derive(Debug, Clone)]
/// # Valid forms
/// (undefined)
/// T
/// T<A: T, B = T, C: T = T>
/// T | T
/// T + T
/// T for T
/// fun (T) -> T | (T) => T
pub enum Typing {
  Undefined,
  Single(Type),
  Tuple(Vec<Typing>),
  // List(Box<Typing>),
  // Map(Box<Typing>, Box<Typing>), ?????????????????????????
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
    if matches!(tokens.peek_token(), Tk::Op(Op::Pipe)) {
      tokens.next();
    }

    let mut name = Vec::with_capacity(1);
    match tokens.peek_token() {
      Tk::Ident(ident) => {
        tokens.next();
        name.push(ident.clone());
        while let Tk::Op(Op::DoubleColon) = tokens.peek_token() {
          tokens.next();
          match tokens.next_token() {
            Tk::Ident(ident) => name.push(ident.clone()),
            _ => syntax_err!("expected type name")
          }
        }
      }
      Tk::Keyword(Kw::Fun) => {
        tokens.next();
        syntax_err!("functions as types not yet implemented");
      }
      Tk::Op(Op::LeftAngle) => {
        tokens.next();
        syntax_err!("`<T>` syntax not yet implemented");
      }
      Tk::Keyword(Kw::Impl) => {
        tokens.next();
        syntax_err!("`impl T` syntax not yet implemented");
      }
      _ => {}
    }

    let generics = parse_generics(tokens);
    if let Tk::Keyword(Kw::For) = tokens.peek() {
      tokens.next();
      syntax_err!("`{} for T` syntax not yet implemented", name.join("::"));
    }
    if let Tk::Op(Op::Pipe) = tokens.peek() {
      tokens.next();
      syntax_err!("`{} | T` syntax not yet implemented", name.join("::"));
    }
    if let Tk::Op(Op::Plus) = tokens.peek() {
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

/// Join a list of displayable items
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
