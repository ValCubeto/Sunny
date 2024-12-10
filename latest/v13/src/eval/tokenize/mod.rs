mod keywords;
mod tokens;

use keywords::Keyword;
use tokens::Token as Tk;
use crate::{debug, debug_msg};

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
pub fn tokenize(input: String) -> Vec<Tk> {
  let mut tokens = Vec::new();
  let mut chars = input.chars().peekable();
  while let Some(ch) = chars.next() {
    println!();
    debug_msg!("Current char: {ch:?}");
    match ch {
      ' ' | '\t' => {
        while let Some(&ch) = chars.peek() {
          if !matches!(ch, ' ' | '\t') {
            break;
          }
          chars.next();
        }
      },
      '\n' | '\r' => {
        let mut skipped = String::from(ch);
        // Collect all the new line characters
        while let Some(&ch) = chars.peek() {
          if !matches!(ch, '\n' | '\r') {
            break;
          }
          skipped.push(ch);
          chars.next();
        }
        debug_msg!("Skipped: {skipped:?}");
        tokens.push(Tk::NewLine)
      },
      'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
        let mut word = String::from(ch);
        while let Some(&ch) = chars.peek() {
          if !matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
            break;
          }
          word.push(ch);
          chars.next();
        }
        let token = match Keyword::parse(&word) {
          Some(kw) => Tk::Keyword(kw),
          None => Tk::Ident(word),
        };
        debug_msg!("Word: {token:?}");
        tokens.push(token);
      },
      '(' => tokens.push(Tk::LeftParen),
      ')' => tokens.push(Tk::RightParen),
      '{' => tokens.push(Tk::LeftBrace),
      '}' => tokens.push(Tk::RightBrace),
      '<' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::LessEqual);
          continue;
        }
        tokens.push(Tk::LeftAngle);
      },
      '>' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::GreaterEqual);
          continue;
        }
        tokens.push(Tk::RightAngle);
      },
      '[' => tokens.push(Tk::LeftBracket),
      ']' => tokens.push(Tk::RightBracket),
      '.' => {
        if chars.peek() == Some(&'.') {
          chars.next();
          if chars.peek() == Some(&'.') {
            chars.next();
            tokens.push(Tk::TripleDot);
            continue;
          }
          tokens.push(Tk::DoubleDot);
          continue;
        }
        tokens.push(Tk::Dot);
      },
      ':' => {
        if chars.peek() == Some(&':') {
          chars.next();
          tokens.push(Tk::DoubleColon);
          continue;
        }
        tokens.push(Tk::Colon);
      }
      ',' => tokens.push(Tk::Comma),
      ';' => {
        // Collect multiple semicolons
        while let Some(&ch) = chars.peek() {
          if ch != ';' {
            break;
          }
          chars.next();
        }
        tokens.push(Tk::Semicolon);
      },
      '+' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::AddAssign);
          continue;
        }
        tokens.push(Tk::Plus);
      },
      '-' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::SubAssign);
          continue;
        }
        tokens.push(Tk::Minus);
      },
      '*' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::MulAssign);
          continue;
        }
        tokens.push(Tk::Star);
      },
      '/' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::DivAssign);
          continue;
        }
        tokens.push(Tk::Slash);
      },
      '%' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::ModAssign);
          continue;
        }
        tokens.push(Tk::Percent);
      },
      '^' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::XorAssign);
          continue;
        }
        tokens.push(Tk::Xor);
      },
      '&' => {
        match chars.peek() {
          // `&&`
          Some(&'&') => {
            chars.next();
            // `&&=`
            if chars.peek() == Some(&'=') {
              chars.next();
              tokens.push(Tk::LogicalAndAssign);
              continue;
            }
            tokens.push(Tk::LogicalAnd);
            continue;
          }
          // `&=`
          Some(&'=') => {
            tokens.push(Tk::AndAssign);
            continue;
          }
          _ => {}
        }
        tokens.push(Tk::And);
      },
      '|' => {
        match chars.peek() {
          // `||`
          Some(&'|') => {
            chars.next();
            // `||=`
            if chars.peek() == Some(&'=') {
              chars.next();
              tokens.push(Tk::LogicalOrAssign);
              continue;
            }
            tokens.push(Tk::LogicalOr);
            continue;
          }
          // `|=`
          Some(&'=') => {
            tokens.push(Tk::OrAssign);
            continue;
          }
          _ => {}
        }
        tokens.push(Tk::Or);
      },
      '=' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::DoubleEqual);
          continue;
        }
        tokens.push(Tk::Equal);
      },
      '"' => {
        let mut string = String::new();
        while let Some(&ch) = chars.peek() {
          chars.next();
          match ch {
            '"' => {
              tokens.push(Tk::String(string));
              break;
            }
            '\n' => {
              eprintln!("Unterminated string");
              std::process::exit(1);
            }
            _ => string.push(ch)
          }
        }
      }
      _ => {
        eprintln!("Unexpected token: {ch:?}");
        std::process::exit(1);
      },
    }
  }
  tokens
}
