pub mod keywords;
pub mod tokens;
pub mod numbers;
pub mod strings;

use std::{str::Chars, sync::atomic::Ordering};
use std::sync::atomic::AtomicUsize;
use keywords::parse_word;
use numbers::{ Number, parse_bin, parse_hex, parse_number };
use peekmore::{ PeekMore, PeekMoreIterator };
use strings::{ parse_char, parse_raw_string, parse_string, FString };
use tokens::{ Operator as Op, Token as Tk };

pub static LINE: AtomicUsize = AtomicUsize::new(1);
pub static COLUMN: AtomicUsize = AtomicUsize::new(1);
pub static TOK_LEN: AtomicUsize = AtomicUsize::new(1);

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
    self.saved_pos.line = LINE.load(Ordering::Relaxed);
    self.saved_pos.column = COLUMN.load(Ordering::Relaxed);
  }
  /// Does not update the position
  pub fn next_raw(&mut self) -> Option<char> {
    self.iterator.next()
  }
  pub fn advance_cursor(&mut self, curr: Option<char>) {
    match curr {
      Some('\n') => {
        COLUMN.store(1, Ordering::Relaxed);
        LINE.fetch_add(1, Ordering::Relaxed);
      }
      Some(..) => {
        COLUMN.fetch_add(1, Ordering::Relaxed);
      }
      None => {}
    }
  }
  /// Updates the position
  pub fn next(&mut self) -> Option<char> {
    let curr = self.next_raw();
    self.advance_cursor(curr);
    curr
  }
  pub fn peek(&mut self) -> Option<char> {
    self.iterator.peek().copied()
  }
  pub fn peek_amount(&mut self, amount: usize) -> &[Option<char>] {
    self.iterator.peek_amount(amount)
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
  pub line: usize,
  pub column: usize,
  pub tok_len: usize,
}

pub type Tokens = Vec<(Position, Tk)>;

pub fn tokenize(input: String) -> Tokens {
  let mut tokens: Tokens = Vec::new();
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
        if chars.peek() != Some('\'') {
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
      'f' | 'F' => match chars.peek() {
        Some('"') => {
          chars.next();
          let (fstring, len) = FString::parse(&mut chars);
          push!(Tk::FString(fstring), len + 3);
        }
        Some(..) | None => {
          let (word, len) = parse_word(&mut chars, ch);
          push!(word, len);
        }
      }
      'r' | 'R' => match chars.peek() {
        Some('"') => {
          chars.next();
          let (raw_string, len) = parse_raw_string(&mut chars);
          push!(Tk::String(raw_string), len + 3);
        }
        Some(..) | None => {
          let (word, len) = parse_word(&mut chars, ch);
          push!(word, len);
        }
      }
      'a'..='z' | 'A'..='Z' | '_' => {
        let (word, len) = parse_word(&mut chars, ch);
        push!(word, len);
      }
      '(' => push!(Tk::LeftParen),
      ')' => push!(Tk::RightParen),
      '{' => {
        if chars.peek() == Some('{') {
          push!(Tk::DoubleLeftBrace, 2);
          continue;
        }
        push!(Tk::LeftBrace);
      }
      '}' => {
        if chars.peek() == Some('}') {
          push!(Tk::DoubleRightBrace, 2);
          continue;
        }
        push!(Tk::RightBrace);
      }
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
          if chars.peek() == Some('=') {
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
          if chars.peek() == Some('=') {
            chars.next();
            push!(Tk::Op(Op::RightShiftAssign), 3);
            continue;
          }
          push!(Tk::Op(Op::DoubleRightAngle), 2);
        }
        _ => push!(Tk::Op(Op::RightAngle))
      }
      '.' => {
        if chars.peek() == Some('.') {
          chars.next();
          if chars.next() != Some('.') {
            syntax_err!("expected a dot here");
          }
          push!(Tk::Op(Op::TripleDot), 3);
          continue;
        }
        push!(Tk::Op(Op::Dot));
      }
      ':' => {
        if chars.peek() == Some(':') {
          chars.next();
          push!(Tk::Op(Op::DoubleColon), 2);
          continue;
        }
        push!(Tk::Colon);
      }
      ';' => {
        // Collect multiple semicolons
        while let Some(ch) = chars.peek() {
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
        Some(d) if d.is_ascii_digit() => {
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
        Some('>') => {
          chars.next();
          push!(Tk::Arrow, 2);
        }
        Some(d) if d.is_ascii_digit() => {
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
          while let Some(ch) = chars.peek() {
            if ch == '\n' {
              break;
            }
            chars.next();
          }
        }
        Some('*') => {
          chars.next();
          while let Some(ch) = chars.peek() {
            if ch == '*' {
              chars.next();
              if chars.peek() == Some('/') {
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
        if chars.peek() == Some('=') {
          chars.next();
          push!(Tk::Op(Op::ModAssign), 2);
          continue;
        }
        push!(Tk::Op(Op::Percent));
      }
      '^' => {
        if chars.peek() == Some('=') {
          chars.next();
          push!(Tk::Op(Op::XorAssign), 2);
          continue;
        }
        push!(Tk::Op(Op::Caret));
      }
      '&' => match chars.peek() {
        // `&&`
        Some('&') => {
          chars.next();
          // `&&=`
          if chars.peek() == Some('=') {
            chars.next();
            push!(Tk::Op(Op::LogicalAndAssign), 3);
            continue;
          }
          push!(Tk::Op(Op::DoubleAmpersand), 2);
        }
        // `&=`
        Some('=') => {
          chars.next();
          push!(Tk::Op(Op::AndAssign), 2);
        }
        _ => push!(Tk::Op(Op::Ampersand))
      }
      '|' => match chars.peek() {
        // `||`
        Some('|') => {
          chars.next();
          // `||=`
          if chars.peek() == Some('=') {
            chars.next();
            push!(Tk::Op(Op::LogicalOrAssign), 3);
            continue;
          }
          push!(Tk::Op(Op::DoublePipe), 2);
        }
        // `|=`
        Some('=') => {
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
          push!(Tk::FatArrow, 2);
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
        Some(d) if d.is_ascii_digit() => {
          // Skip this leading zero
          chars.next();
          let (num, len) = parse_number(&mut chars, d);
          push!(Tk::Number(num), len + 1);
        }
        Some(c) if c.is_ascii_alphabetic() => syntax_err!("unexpected character {c:?}"),
        _ => push!(Tk::Number(Number::Int("0".to_owned())))
      }
      d @ '1'..='9' => {
        let (num, len) = parse_number(&mut chars, d);
        push!(Tk::Number(num), len);
      }
      _ => {
        COLUMN.fetch_sub(1, Ordering::Relaxed);
        syntax_err!("unexpected token {ch:?}")
      }
    } // match
    chars.save_pos();
  } // while
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
