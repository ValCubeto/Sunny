#[allow(unused)]
#[derive(Debug)]
/// An intermediate value, not evaluated yet
pub enum Value {
  Char(char),
  String(String),
  Ident(String),
  // Number,
  // FString,
  // RString,
  // CString,
}
