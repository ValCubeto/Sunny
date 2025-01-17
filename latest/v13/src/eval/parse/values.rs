use std::fmt;
use crate::eval::tokenize::{ numbers::Number, strings::ParsedFString };
use super::functions::Function;

#[allow(unused)]
#[derive(Debug)]
/// An intermediate value, not evaluated yet
pub enum Value {
  Char(char),
  String(String),
  FString(ParsedFString),
  // CString,
  Ident(String),
  Number(Number),
  Function(Function)
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Char(c) => write!(f, "{c:?}"),
      Value::String(s) => write!(f, "{s:?}"),
      Value::FString(s) => write!(f, "{s:?}"),
      Value::Ident(s) => write!(f, "{s}"),
      Value::Number(n) => write!(f, "{n}"),
      Value::Function(func) => write!(f, "{func:#?}")
    }
  }
}
