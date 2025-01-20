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
          _ => Typing::undefined()
        };
        let default_val = match tokens.peek_token() {
          Tk::Op(Op::Equal) => {
            tokens.next();
            Typing::parse(tokens)
          }
          _ => Typing::undefined()
        };
        generics.push(GenericParam {
          name: name.clone(),
          typing,
          default_val
        });
        if !tokens.comma_sep() {
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
pub enum Type {
  Undefined,
  /// `path::to::Type<A: X, B = Y>`
  Ref {
    name: Vec<String>,
    generics: Vec<GenericParam>,
  },
  /// `fun (A, B) -> C`
  Function {
    args: Vec<Typing>,
    output: Box<Type>
  }
}

impl Type {
  pub fn parse(tokens: &mut Tokens) -> Type {
    match tokens.next_token() {
      Tk::Ident(ident) => {
        let mut name = Vec::with_capacity(1);
        name.push(ident.clone());
        while let Tk::Op(Op::DoubleColon) = tokens.peek_token() {
          tokens.next();
          match tokens.next_token() {
            Tk::Ident(ident) => name.push(ident.clone()),
            _ => syntax_err!("expected identifier")
          }
        }
        Type::Ref {
          name,
          generics: parse_generics(tokens)
        }
      }
      Tk::Keyword(Kw::Fun) => {
        syntax_err!("functions as types not yet implemented");
      }
      _ => syntax_err!("expected type")
    }
  }
  pub fn from_function(function: &Function) -> Self {
    Type::Function {
      args: function.params.iter().map(|p| p.typing.clone()).collect(),
      output: Box::new(function.output.clone())
    }
  }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Typing {
  Never,
  Single(Type),
  Tuple(Vec<Typing>),
  List(Box<Typing>),
  And(Vec<Typing>),
  ImplFor(Type, Type),
}

impl Typing {
  #[inline]
  pub fn undefined() -> Self {
    Typing::Single(Type::Undefined)
  }
  pub fn parse(tokens: &mut Tokens) -> Typing {
    match tokens.peek_token() {
      Tk::LeftParen => {
        let mut types = Vec::new();
        tokens.next();
        while let Tk::Ident(..) = tokens.peek_token() {
          let ty = Typing::parse(tokens);
          types.push(ty);
          if !tokens.comma_sep() {
            break;
          }
        }
        match tokens.peek_token() {
          Tk::RightParen => {
            tokens.next();
          }
          other => syntax_err!("unexpected {other}")
        }
        Typing::Tuple(types)
      }
      Tk::Ident(..) | Tk::Keyword(Kw::Fun) => Typing::Single(Type::parse(tokens)),
      other => syntax_err!("unexpected {other}")
    }
  }
}

/// Join a list of displayable items
pub fn join<T: fmt::Display>(iter: impl Iterator<Item = T>, sep: &str) -> String {
  iter.map(|ty| ty.to_string()).collect::<Vec<_>>().join(sep)
}

impl fmt::Display for GenericParam {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", display_param(&self.name, &self.typing, &self.default_val))
  }
}

impl fmt::Display for TypeRef {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.name.join("::"))?;
    if !self.generics.is_empty() {
      write!(f, "<{}>", join(self.generics.iter(), ", "))?;
    }
    Ok(())
  }
}

impl fmt::Display for FunctionType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "fun ({}) -> {}", join(self.args.iter(), ", "), self.output)
  }
}

impl fmt::Display for Typing {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Never => write!(f, "never"),
      Self::List(ty) => write!(f, "{ty}[]"),
      Self::Single(ty) => write!(f, "{ty}"),
      Self::Tuple(tys) => write!(f, "({})", join(tys.iter(), ", ")),
      Self::And(tys) => write!(f, "{}", join(tys.iter(), " + ")),
      Self::ImplFor(idea, ty) => write!(f, "{idea} for {ty}"),
    }
  }
}
