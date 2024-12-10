mod keywords;
mod tokens;
use tokens::Token::{self, *};

/// ```rs
/// fun main() {
///   println("Hello, world!")
/// }
/// ```
/// ...
/// ```rs
/// [
///   Keyword(Fun),
///   Ident("main"),
///   LeftParen,
///   RightParen,
///   LeftBrace,
///   NewLine,
///   Ident("println"),
///   LeftParen,
///   String("Hello, world!"),
///   RightParen,
///   NewLine,
///   RightBrace
/// ]
/// ```
pub fn tokenize(input: String) -> Vec<Token> {
  let mut tokens = Vec::new();
  for ch in input.chars() {
    match ch {
      ' ' | '\t' => {},
      '\n' => tokens.push(NewLine),
      _ => {
        eprintln!("Unexpected token");
        std::process::exit(1);
      },
    }
  }
  tokens
}
