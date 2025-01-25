use std::fmt;
use crate::eval::{
  tokenize::{
    keywords::Keyword as Kw,
    tokens::{ Operator as Op, Token as Tk, TokenIter },
  },
  parse::functions::{ Function, display_param }
};

use super::expressions::Expr;

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

pub fn parse_generics(tokens: &mut TokenIter) -> Vec<GenericParam> {
  let mut generics = Vec::new();
  match tokens.peek_token() {
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
      match tokens.next_token() {
        Tk::Op(Op::RightAngle) => generics,
        other => syntax_err!("unexpected {other}")
      }
    }
    // empty generics
    Tk::Op(Op::Diamond) => {
      tokens.next();
      generics
    }
    _ => generics
  }
}

#[derive(Debug, Clone)]
pub enum Typing {
  Undefined,
  Never,
  /// `&T`
  Ptr(Box<Self>),
  /// `T?`
  Maybe(Box<Self>),
  /// `path::to::T<A: X, B = Y>`
  Ref {
    name: Vec<String>,
    generics: Vec<GenericParam>
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
  List(Box<Self>, Box<Expr>),
  // /// I for T
  // ImplFor(Box<Self>, Box<Self>)
}

impl Typing {
  pub fn parse(tokens: &mut TokenIter) -> Self {
    let mut typing = Self::parse_single(tokens);
    // loop {
    //   match tokens.peek_token() {
    //     // Allow nested options
    //     Tk::Op(Op::Question) => {
    //       tokens.next();
    //       syntax_err!("optional types not yet implemented");
    //     }
    //     Tk::Op(Op::Plus) => {
    //       tokens.next();
    //       syntax_err!("multiple implementations not yet implemented");
    //     }
    //     Tk::Keyword(Kw::For) => {
    //       tokens.next();
    //       syntax_err!("implementations not yet implemented");
    //     }
    //     _ => break
    //   }
    // }
    typing
  }
  pub fn parse_single(tokens: &mut TokenIter) -> Self {
    match tokens.next_token() {
      Tk::Keyword(Kw::Never) => Self::Never,
      Tk::Op(Op::Ampersand) => Self::Ptr(Box::new(Self::parse_single(tokens))),
      Tk::Ident(ident) => {
        let mut name = vec![ident.clone()];
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
      // (A, B)
      Tk::LeftParen => {
        let mut types = Vec::new();
        while !matches!(tokens.next_token(), Tk::RightParen) {
          types.push(Self::parse(tokens));
          if !tokens.comma_sep() {
            break;
          }
        }
        Typing::Tuple(types)
      }
      Tk::LeftBracket => {
        let ty = Self::parse(tokens);
        let len = match tokens.peek_token() {
          Tk::Semicolon => {
            tokens.next();
            Expr::parse(tokens)
          }
          _ => Expr::None
        };
        match tokens.next_token() {
          Tk::RightBracket => Typing::List(Box::new(ty), Box::new(len)),
          other => syntax_err!("unexpected {other}")
        }
      }
      // <T>
      Tk::Op(Op::LeftAngle) => {
        let ty = Self::parse_single(tokens);
        match tokens.next_token() {
          Tk::Op(Op::RightAngle) => ty,
          other => syntax_err!("unexpected {other}")
        }
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
      Self::Ptr(ty) => write!(f, "&{ty}"),
      Self::Maybe(ty) => write!(f, "{ty}?"),
      Self::Ref { name, generics } => {
        write!(f, "{}", name.join("::"))?;
        if !generics.is_empty() {
          write!(f, "<{}>", join(generics.iter(), ", "))?;
        }
        Ok(())
      }
      Self::Function { args, output } => write!(f, "({}) -> {}", join(args.iter(), ", "), output),
      Self::List(ty, len) => {
        if matches!(len.as_ref(), Expr::None) {
          write!(f, "[{ty}]")
        } else {
          write!(f, "[{ty}; {len}]")
        }
      }
      Self::Tuple(tys) => write!(f, "({})", join(tys.iter(), ", ")),
      Self::And(tys) => write!(f, "{}", join(tys.iter(), " + ")),
      // Self::ImplFor(idea, ty) => write!(f, "{idea} for {ty}"),
    }
  }
}
