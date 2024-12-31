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
