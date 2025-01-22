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
pub enum Typing {
  Undefined,
  Never,
  /// `path::to::Type<A: X, B = Y>`
  Ref {
    name: Vec<String>,
    generics: Vec<GenericParam>,
  },
  /// `fun (A, B) -> C`
  Function {
    args: Vec<Self>,
    output: Box<Self>
  },
  /// (A, B, C)
  Tuple(Vec<Self>),
  /// A + B + C
  And(Vec<Self>),
  /// T[]
  List(Box<Self>),
  // /// I for T
  // ImplFor(Box<Self>, Box<Self>)
}

// <fun (A, B) -> C[]>[]
impl Typing {
  pub fn parse(tokens: &mut Tokens) -> Typing {
    match tokens.peek_token() {
      Tk::LeftParen => {
        let mut types = Vec::new();
        tokens.next();
        loop {
          types.push(Typing::parse(tokens));
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
      _ => Self::parse_single(tokens)
    }
  }
  pub fn parse_single(tokens: &mut Tokens) -> Self {
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
        Self::Ref {
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
    Self::Function {
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
      Self::Never => write!(f, "never"),
      Self::Ref { name, generics } => {
        write!(f, "{}", name.join("::"))?;
        if !generics.is_empty() {
          write!(f, "<{}>", join(generics.iter(), ", "))?;
        }
        Ok(())
      }
      Self::Function { args, output } => write!(f, "fun ({}) -> {}", join(args.iter(), ", "), output),
      Self::List(ty) => write!(f, "{ty}[]"),
      Self::Tuple(tys) => write!(f, "({})", join(tys.iter(), ", ")),
      Self::And(tys) => write!(f, "{}", join(tys.iter(), " + ")),
      Self::ImplFor(idea, ty) => write!(f, "{idea} for {ty}"),
    }
  }
}
