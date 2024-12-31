use crate::eval::tokenize::number::Number;

#[allow(unused)]
#[derive(Debug)]
/// An intermediate value, not evaluated yet
pub enum Value {
  Char(char),
  String(String),
  Ident(String),
  Number(Number),
  // FString,
  // RString,
  // CString,
}

impl std::fmt::Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Char(c) => write!(f, "{c:?}"),
      Value::String(s) => write!(f, "{s:?}"),
      Value::Ident(s) => write!(f, "{s}"),
      Value::Number(n) => write!(f, "{n}"),
    }
  }
}
