use crate::eval::tokenize::tokenize;

use super::{ skip_spaces, tokens::Token, CharsIter, Position };

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
          if chars.peek() != Some(&'{') {
            syntax_err!("expected '{{' after '\\u'");
          }
          chars.next();
          len += skip_spaces(chars);
          let mut hex = String::new();
          while let Some(&ch) = chars.peek() {
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
          let ch = char::from_u32(code).unwrap_or_else(|| syntax_err!("invalid escape sequence {hex:?}"));
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
  while let Some(&ch) = chars.peek() {
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

pub fn parse_fstring(chars: &mut CharsIter) -> (FString, usize) {
  let mut literals = vec![String::new()];
  let mut inserted = vec![];
  let mut curr_literal: &mut String = literals.last_mut().unwrap();
  let mut len = 0;
  while let Some(&ch) = chars.peek() {
    match ch {
      '{' => {
        chars.next();
        len += 1;
        literals.push(String::new());
        curr_literal = literals.last_mut().unwrap();
        // let inserted_code = String::new();
        match chars.next() {
          Some('}') => syntax_err!("empty formatting"),
          Some('\n') | None => syntax_err!("unclosed formatting"),
          Some(c) => {
            todo!();
            let mut insert = String::from(c);
            // collect the piece of inserted code until '}' is found
            // while let Some(ch)
            let tokens = tokenize(insert);
            inserted.push(tokens);
          }
        }
      }
      '"' => {
        chars.next();
        let fstring = FString {
          literals,
          inserted
        };
        return (fstring, len);
      }
      '\n' => break,
      _ => {
        let (ch, ch_len) = parse_char(chars);
        len += ch_len;
        curr_literal.push(ch);
      }
    }
  }
  syntax_err!("unclosed string");
}
