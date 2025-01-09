pub mod keywords;
pub mod tokens;
pub mod numbers;
pub mod strings;
use std::str::Chars;
use keywords::parse_word;
use numbers::{ Number, parse_bin, parse_hex, parse_number };
use peekmore::{ PeekMore, PeekMoreIterator };
use strings::{parse_char, parse_string};
use tokens::{ Operator as Op, Token as Tk };

pub static mut LINE: usize = 1;
pub static mut COLUMN: usize = 1;
pub static mut TOK_LEN: usize = 1;

pub struct CharsIter<'a> {
  iterator: PeekMoreIterator<Chars<'a>>,
  pub saved_pos: Position
}
impl<'a> CharsIter<'a> {
  pub fn new(iterator: PeekMoreIterator<Chars<'a>>) -> Self {
    CharsIter {
      iterator,
      saved_pos: Position { line: 1, column: 1, tok_len: 1 }
    }
  }
  /// Save the current position at the start of a token
  pub fn save_pos(&mut self) {
    unsafe {
      self.saved_pos.line = LINE;
      self.saved_pos.column = COLUMN;
    }
  }
  pub fn next(&mut self) -> Option<char> {
    let curr = self.iterator.next();
    unsafe {
      match curr {
        Some('\n') => {
          COLUMN = 1;
          LINE += 1;
        }
        Some(_) => {
          COLUMN += 1;
        }
        None => {}
      }
    }
    curr
  }
  pub fn peek(&mut self) -> Option<&char> {
    self.iterator.peek()
  }
  pub fn peek_amount(&mut self, amount: usize) -> &[Option<char>] {
    self.iterator.peek_amount(amount)
  }
}

#[derive(Clone, Copy)]
pub struct Position {
  pub line: usize,
  pub column: usize,
  pub tok_len: usize,
}

pub fn tokenize(input: String) -> Vec<(Position, Tk)> {
  let mut tokens: Vec<(Position, Tk)> = Vec::new();
  let mut chars = CharsIter::new(input.chars().peekmore());
  /// `push(token: Token, len: usize = 1)`
  macro_rules! push {
    ($tk:expr) => {{
      tokens.push((chars.saved_pos, $tk));
    }};
    ($tk:expr, $len:expr) => {{
      let mut pos = chars.saved_pos;
      pos.tok_len = $len;
      tokens.push((pos, $tk));
    }};
  }
  chars.save_pos();
  while let Some(ch) = chars.next() {
    match ch {
      ' ' | '\t' => {
        let _ = skip_spaces(&mut chars);
      }
      '\'' => {
        let (ch, len) = parse_char(&mut chars);
        if chars.peek() != Some(&'\'') {
          syntax_err!("expected a closing single quote here");
          // "If you meant to write a string literal, use double quotes"
        }
        chars.next();
        push!(Tk::Char(ch), len + 2);
      }
      '"' => {
        let (string, len) = parse_string(&mut chars);
        push!(Tk::String(string), len + 2);
      }
      'f' | 'F' => {
        if chars.peek() != Some(&'"') {
          let (word, len) = parse_word(&mut chars, ch);
          push!(word, len);
          continue;
        }
        // if chars.peek() == Some(&'"') {
        //   chars.next();
        //   let (fstring, len) = parse_fstring(&mut chars);
        //   push!(Tk::FString(fstring), len + 2);
        //   continue;
        // }
      }
      'a'..='z' | 'A'..='Z' | '_' => {
        let (word, len) = parse_word(&mut chars, ch);
        push!(word, len);
      }
      '(' => push!(Tk::LeftParen),
      ')' => push!(Tk::RightParen),
      '{' => push!(Tk::LeftBrace),
      '}' => push!(Tk::RightBrace),
      '[' => push!(Tk::LeftBracket),
      ']' => push!(Tk::RightBracket),
      ',' => push!(Tk::Comma),
      '?' => push!(Tk::Op(Op::Question)),
      '!' => match chars.peek() {
        Some('=') => {
          chars.next();
          push!(Tk::Op(Op::NotEqual), 2);
          continue;
        }
        _ => push!(Tk::Op(Op::Bang))
      }
      '\n' | '\r' => {
        // Collect all the new line characters
        while let Some('\n' | '\r') = chars.peek() {
          chars.next();
        }
        if !matches!(tokens.last(), Some((_, Tk::NewLine))) {
          push!(Tk::NewLine);
        }
      }
      '<' => match chars.peek() {
        Some('>') => {
          chars.next();
          push!(Tk::Op(Op::Diamond), 2);
        }
        Some('=') => {
          chars.next();
          push!(Tk::Op(Op::LessOrEqual), 2);
        }
        Some('<') => {
          chars.next();
          if chars.peek() == Some(&'=') {
            chars.next();
            push!(Tk::Op(Op::LeftShiftAssign), 3);
            continue;
          }
          push!(Tk::Op(Op::DoubleLeftAngle), 2);
        }
        _ => push!(Tk::Op(Op::LeftAngle))
      }
      '>' => match chars.peek() {
        Some('=') => {
          chars.next();
          push!(Tk::Op(Op::GreaterOrEqual), 2);
        }
        Some('>') => {
          chars.next();
          if chars.peek() == Some(&'=') {
            chars.next();
            push!(Tk::Op(Op::RightShiftAssign), 3);
            continue;
          }
          push!(Tk::Op(Op::DoubleRightAngle), 2);
        }
        _ => push!(Tk::Op(Op::RightAngle))
      }
      '.' => {
        if chars.peek() == Some(&'.') {
          chars.next();
          if chars.peek() == Some(&'.') {
            chars.next();
            push!(Tk::Op(Op::TripleDot), 3);
            continue;
          }
          push!(Tk::Op(Op::DoubleDot), 2);
          continue;
        }
        push!(Tk::Op(Op::Dot));
      }
      ':' => {
        if chars.peek() == Some(&':') {
          chars.next();
          push!(Tk::Op(Op::DoubleColon), 2);
          continue;
        }
        push!(Tk::Colon);
      }
      ';' => {
        // Collect multiple semicolons
        while let Some(&ch) = chars.peek() {
          if ch != ';' {
            break;
          }
          chars.next();
          skip_spaces(&mut chars);
        }
        if !matches!(tokens.last(), Some((_, Tk::Semicolon))) {
          push!(Tk::Semicolon);
        }
      }
      '+' => match chars.peek() {
        Some('=') => {
          chars.next();
          push!(Tk::Op(Op::AddAssign), 2);
        }
        Some(&d) if d.is_ascii_digit() => {
          // Skip the plus sign
          chars.next();
          let (num, len) = parse_number(&mut chars, d);
          push!(Tk::Number(num), len + 1);
        }
        _ => push!(Tk::Op(Op::Plus))
      }
      '-' => match chars.peek() {
        Some('=') => {
          chars.next();
          push!(Tk::Op(Op::SubAssign), 2);
        }
        Some(&d) if d.is_ascii_digit() => {
          let (num, len) = parse_number(&mut chars, '-');
          push!(Tk::Number(num), len);
        }
        _ => push!(Tk::Op(Op::Minus))
      }
      '*' => match chars.peek() {
        Some('=') => {
          chars.next();
          push!(Tk::Op(Op::MulAssign), 2);
        }
        _ => push!(Tk::Op(Op::Star))
      }
      '/' => match chars.peek() {
        Some('=') => {
          chars.next();
          push!(Tk::Op(Op::DivAssign), 2);
        }
        Some('/') => {
          // Inline comments
          chars.next();
          while let Some(&ch) = chars.peek() {
            if ch == '\n' {
              break;
            }
            chars.next();
          }
        }
        Some('*') => {
          chars.next();
          while let Some(&ch) = chars.peek() {
            if ch == '*' {
              chars.next();
              if chars.peek() == Some(&'/') {
                chars.next();
                break;
              }
              continue;
            }
            chars.next();
          }
          if chars.peek().is_none() {
            syntax_err!("Unclosed comment!!");
          }
        }
        _ => push!(Tk::Op(Op::Slash))
      }
      '%' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          push!(Tk::Op(Op::ModAssign), 2);
          continue;
        }
        push!(Tk::Op(Op::Percent));
      }
      '^' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          push!(Tk::Op(Op::XorAssign), 2);
          continue;
        }
        push!(Tk::Op(Op::Caret));
      }
      '&' => match chars.peek() {
        // `&&`
        Some(&'&') => {
          chars.next();
          // `&&=`
          if chars.peek() == Some(&'=') {
            chars.next();
            push!(Tk::Op(Op::LogicalAndAssign), 3);
            continue;
          }
          push!(Tk::Op(Op::DoubleAmpersand), 2);
        }
        // `&=`
        Some(&'=') => {
          chars.next();
          push!(Tk::Op(Op::AndAssign), 2);
        }
        _ => push!(Tk::Op(Op::Ampersand))
      }
      '|' => match chars.peek() {
        // `||`
        Some(&'|') => {
          chars.next();
          // `||=`
          if chars.peek() == Some(&'=') {
            chars.next();
            push!(Tk::Op(Op::LogicalOrAssign), 3);
            continue;
          }
          push!(Tk::Op(Op::DoublePipe), 2);
        }
        // `|=`
        Some(&'=') => {
          chars.next();
          push!(Tk::Op(Op::OrAssign), 2);
        }
        _ => push!(Tk::Op(Op::Pipe))
      }
      '=' => match chars.peek() {
        Some('=') => {
          chars.next();
          push!(Tk::Op(Op::DoubleEqual), 2);
          continue;
        }
        Some('>') => {
          chars.next();
          push!(Tk::Arrow);
          continue;
        }
        _ => push!(Tk::Op(Op::Equal))
      }
      '0' => match chars.peek() {
        Some('x' | 'X') => {
          chars.next();
          let (num, len) = parse_hex(&mut chars);
          push!(Tk::Number(num), len);
        }
        Some('b' | 'B') => {
          chars.next();
          let (num, len) = parse_bin(&mut chars);
          push!(Tk::Number(num), len);
        }
        Some(&d) if d.is_ascii_digit() => {
          // Skip this leading zero
          chars.next();
          let (num, len) = parse_number(&mut chars, d);
          push!(Tk::Number(num), len + 1);
        }
        Some(&c) if c.is_ascii_alphabetic() => syntax_err!("unexpected character {c:?}"),
        _ => push!(Tk::Number(Number::Int("0".to_owned())))
      }
      d @ '1'..='9' => {
        let (num, len) = parse_number(&mut chars, d);
        push!(Tk::Number(num), len);
      }
      _ => syntax_err!("unexpected token: {ch:?}")
    } // match
    chars.save_pos();
  } // while
  push!(Tk::EoF);
  tokens
}

/// Returns the number of skipped spaces in case of any token that contains spaces
pub fn skip_spaces(chars: &mut CharsIter) -> usize {
  let mut len = 0;
  while let Some(' ' | '\t') = chars.peek() {
    len += 1;
    chars.next();
  }
  len
}
