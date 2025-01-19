use std::fmt;
use peekmore::PeekMore;
use crate::eval::parse::expressions::Expr;
use crate::eval::tokenize::{
  tokenize,
  skip_spaces,
  tokens::{ Token, Tokens },
  CharsIter,
  Position
};

pub fn parse_raw_string(chars: &mut CharsIter) -> (String, usize) {
  let mut string = String::new();
  while let Some(ch) = chars.next() {
    if ch == '"' {
      break;
    }
    if ch == '\n' {
      syntax_err!("unterminated string literal");
    }
    string.push(ch);
    chars.next();
  }
  let len = string.len();
  (string, len)
}

pub fn parse_char(chars: &mut CharsIter) -> (char, usize) {
  match chars.next() {
    Some('\'') => syntax_err!("empty character literal"),
    Some('\n') | None => syntax_err!("unterminated character literal"),
    Some('\\') => {
      match chars.next() {
        None => syntax_err!("unterminated escape sequence"),
        Some('n') => ('\n', 2),
        Some('r') => ('\r', 2),
        Some('t') => ('\t', 2),
        Some('0') => ('\0', 2),
        Some('e') => ('\x1b', 2),
        Some('u') => {
          // "\u{}"
          let mut len = 4;
          if chars.peek() != Some('{') {
            syntax_err!("expected '{{' after '\\u'");
          }
          chars.next();
          len += skip_spaces(chars);
          let mut hex = String::new();
          while let Some(ch) = chars.peek() {
            match ch {
              '}' | ' ' | '\t' => break,
              '\n' => syntax_err!("unterminated escape sequence"),
              ch if ch.is_ascii_hexdigit() => {
                chars.next();
                hex.push(ch);
              }
              _ => syntax_err!("invalid hex in escape sequence")
            }
          }
          if hex.is_empty() {
            syntax_err!("empty escape sequence");
          }
          len += hex.len();
          let code = match u32::from_str_radix(&hex, 16) {
            Ok(code) => code,
            Err(why) => syntax_err!("invalid escape sequence {hex:?} ({why})")
          };
          let ch = char::from_u32(code).unwrap_or_else(|| {
            syntax_err!("invalid escape sequence {hex:?}");
          });
          len += skip_spaces(chars);
          if chars.next() != Some('}') {
            syntax_err!("expected right brace after escape sequence");
          }
          (ch, len)
        }
        Some(other) => (other, 2)
      }
    }
    Some(other) => (other, 1)
  }
}

pub fn parse_string(chars: &mut CharsIter) -> (String, usize) {
  let mut string = String::new();
  let mut len = 0;
  while let Some(ch) = chars.peek() {
    if ch == '"' {
      chars.next();
      return (string, len);
    }
    if ch == '\n' {
      break;
    }
    let (ch, ch_len) = parse_char(chars);
    len += ch_len;
    string.push(ch);
  }
  syntax_err!("unclosed string");
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct FString {
  pub literals: Vec<String>,
  pub inserted: Vec<Vec<(Position, Token)>>
}

#[allow(unused)]
#[derive(Debug)]
pub struct ParsedFString {
  pub literals: Vec<String>,
  pub inserted: Vec<Expr>
}

impl fmt::Display for FString {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "f\"")?;
    for (literal, inserted) in self.literals.iter().zip(self.inserted.iter()) {
      write!(f, "{literal}{{{}}}", inserted.iter()
        .map(|(_, token)| format!("{token:?}"))
        .collect::<Vec<String>>()
        .join(", ")
      )?;
    }
    if self.literals.len() > self.inserted.len() {
      write!(f, "{}", self.literals.last().unwrap())?;
    }
    write!(f, "\"")
  }
}

// Accept nested braces
fn collect_inserted_code(chars: &mut CharsIter, insert: &mut String) {
  while let Some(ch) = chars.next_raw() {
    insert.push(ch);
    if ch == '{' {
      collect_inserted_code(chars, insert);
      continue;
    }
    if ch == '}' {
      break;
    }
  }
}

impl FString {
  /// Creates an FString from the chars iterator
  pub fn parse(chars: &mut CharsIter) -> (FString, usize) {
    let mut literals = vec![String::new()];
    let mut curr_literal: &mut String = unsafe {
      literals.last_mut().unwrap_unchecked()
    };
    let mut inserted = vec![];
    let mut len = 0;
    while let Some(ch) = chars.peek() {
      match ch {
        '"' => {
          chars.next();
          let fstring = FString {
            literals,
            inserted
          };
          return (fstring, len);
        }
        '\n' => break,
        '{' => {
          chars.next();
          let mut insert = String::new();
          while let Some(ch) = chars.peek() {
            if ch == '}' {
              break;
            }
            // The position is advanced by `tokenize`
            chars.next_raw();
            insert.push(ch);
            if ch == '{' {
              collect_inserted_code(chars, &mut insert);
            }
          }
          let ch = chars.next_raw();
          if ch != Some('}') {
            for ch in insert.chars() {
              chars.advance_cursor(Some(ch));
            }
            syntax_err!("unclosed formatting");
          }
          if chars.peek() != Some('"') {
            literals.push(String::new());
            curr_literal = unsafe {
              literals.last_mut().unwrap_unchecked()
            };
          }
          len += insert.len() + 2;
          let tokens = tokenize(insert);
          chars.advance_cursor(ch);
          if tokens.is_empty() {
            continue;
          }
          inserted.push(tokens);
        }
        _ => {
          let (ch, ch_len) = parse_char(chars);
          len += ch_len;
          curr_literal.push(ch);
        }
      }
    }
    syntax_err!("unclosed string");
  }

  /// Parses the token lists of the inserted expressions
  pub fn to_parsed(&self) -> ParsedFString {
    let mut inserted = Vec::with_capacity(self.inserted.len());
    for tokens in self.inserted.iter() {
      let mut tokens = Tokens::new(tokens.iter().peekmore());
      inserted.push(Expr::parse(&mut tokens));
      match tokens.next() {
        Token::NewLine => {}
        token => syntax_err!("unexpected {token}")
      }
    }
    ParsedFString {
      literals: self.literals.clone(),
      inserted
    }
  }
}
