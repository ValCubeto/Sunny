pub mod keywords;
pub mod tokens;
pub mod number;
use std::str::Chars;
use keywords::Keyword;
use number::{ Number, parse_bin, parse_hex, parse_number };
use peekmore::{ PeekMore, PeekMoreIterator };
use tokens::{ Operator as Op, Token as Tk };

pub static mut LINE: usize = 1;
pub static mut COLUMN: usize = 1;

pub struct CharsIter<'a> {
  iterator: PeekMoreIterator<Chars<'a>>,
  saved_pos: Position
}
impl<'a> CharsIter<'a> {
  pub fn new(iterator: PeekMoreIterator<Chars<'a>>) -> Self {
    CharsIter {
      iterator,
      saved_pos: Position { line: 1, column: 1 }
    }
  }
  /// Save the current position at the start of a token
  pub fn save_pos(&mut self) {
    unsafe {
      self.saved_pos = Position {
        line: LINE,
        column: COLUMN
      }
    }
  }
  pub fn saved_pos(&self) -> Position {
    self.saved_pos
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
}

pub fn tokenize(input: String) -> Vec<(Position, Tk)> {
  let mut tokens: Vec<(Position, Tk)> = Vec::new();
  let mut chars = CharsIter::new(input.chars().peekmore());
  macro_rules! push {
    ($tk:expr) => {
      {
        tokens.push((chars.saved_pos(), $tk));
      }
    }
  }
  chars.save_pos();
  while let Some(ch) = chars.next() {
    match ch {
      ' ' | '\t' => skip_spaces(&mut chars),
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
          push!(Tk::Op(Op::NotEqual));
          continue;
        }
        _ => push!(Tk::Op(Op::Bang))
      }
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
        // Idk how to negate this condition
        if !matches!(tokens.last(), Some((_, Tk::NewLine))) {
          push!(Tk::NewLine);
        }
      }
      '<' => match chars.peek() {
        Some('>') => {
          chars.next();
          push!(Tk::Op(Op::Diamond));
        }
        Some('=') => {
          chars.next();
          push!(Tk::Op(Op::LessOrEqual));
        }
        Some('<') => {
          chars.next();
          if chars.peek() == Some(&'=') {
            chars.next();
            push!(Tk::Op(Op::LeftShiftAssign));
            continue;
          }
          push!(Tk::Op(Op::DoubleLeftAngle));
        }
        _ => push!(Tk::Op(Op::LeftAngle))
      }
      '>' => match chars.peek() {
        Some('=') => {
          chars.next();
          push!(Tk::Op(Op::GreaterOrEqual));
        }
        Some('>') => {
          chars.next();
          if chars.peek() == Some(&'=') {
            chars.next();
            push!(Tk::Op(Op::RightShiftAssign));
            continue;
          }
          push!(Tk::Op(Op::DoubleRightAngle));
        }
        _ => push!(Tk::Op(Op::RightAngle))
      }
      '.' => {
        if chars.peek() == Some(&'.') {
          chars.next();
          if chars.peek() == Some(&'.') {
            chars.next();
            push!(Tk::Op(Op::TripleDot));
            continue;
          }
          push!(Tk::Op(Op::DoubleDot));
          continue;
        }
        push!(Tk::Op(Op::Dot));
      }
      ':' => {
        if chars.peek() == Some(&':') {
          chars.next();
          push!(Tk::Op(Op::DoubleColon));
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
        if let Some((_, Tk::Semicolon)) = tokens.last() {} else {
          push!(Tk::Semicolon);
        }
      }
      '+' => match chars.peek() {
        Some('=') => {
          chars.next();
          push!(Tk::Op(Op::AddAssign));
        }
        Some(&d) if d.is_ascii_digit() => {
          chars.next();
          push!(Tk::Number(parse_number(&mut chars, d)));
        }
        _ => push!(Tk::Op(Op::Plus))
      }
      '-' => match chars.peek() {
        Some('=') => {
          chars.next();
          push!(Tk::Op(Op::SubAssign));
        }
        Some(&d) if d.is_ascii_digit() => {
          push!(Tk::Number(parse_number(&mut chars, '-')));
        }
        _ => push!(Tk::Op(Op::Minus))
      }
      '*' => match chars.peek() {
        Some('=') => {
          chars.next();
          push!(Tk::Op(Op::MulAssign));
        }
        _ => push!(Tk::Op(Op::Star))
      }
      '/' => match chars.peek() {
        Some('=') => {
          chars.next();
          push!(Tk::Op(Op::DivAssign));
        }
        Some('/') => {
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
          push!(Tk::Op(Op::ModAssign));
          continue;
        }
        push!(Tk::Op(Op::Percent));
      }
      '^' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          push!(Tk::Op(Op::XorAssign));
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
            push!(Tk::Op(Op::LogicalAndAssign));
            continue;
          }
          push!(Tk::Op(Op::DoubleAmpersand));
        }
        // `&=`
        Some(&'=') => {
          chars.next();
          push!(Tk::Op(Op::AndAssign));
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
            push!(Tk::Op(Op::LogicalOrAssign));
            continue;
          }
          push!(Tk::Op(Op::DoublePipe));
        }
        // `|=`
        Some(&'=') => {
          chars.next();
          push!(Tk::Op(Op::OrAssign));
        }
        _ => push!(Tk::Op(Op::Pipe))
      }
      '=' => match chars.peek() {
        Some('=') => {
          chars.next();
          push!(Tk::Op(Op::DoubleEqual));
          continue;
        }
        Some('>') => {
          chars.next();
          push!(Tk::Arrow);
          continue;
        }
        _ => push!(Tk::Op(Op::Equal))
      },
      '0' => match chars.peek() {
        Some('x') => {
          chars.next();
          push!(Tk::Number(parse_hex(&mut chars)));
        }
        Some('b') => {
          chars.next();
          push!(Tk::Number(parse_bin(&mut chars)));
        }
        Some(&d) if d.is_ascii_digit() => {
          chars.next(); // duplicated 'd'
          push!(Tk::Number(parse_number(&mut chars, d)));
        }
        Some(&c) if c.is_ascii_alphabetic() => syntax_err!("unexpected character {c:?}"),
        _ => push!(Tk::Number(Number::Int("0".to_owned())))
      }
      d @ '1'..='9' => push!(Tk::Number(parse_number(&mut chars, d))),
      'a'..='z' | 'A'..='Z' | '_' => {
        let mut word = String::from(ch);
        while let Some(&ch) = chars.peek() {
          // TODO: add more word characters
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
        push!(token);
      }
      '"' => push!(Tk::String(parse_string(&mut chars))),
      _ => syntax_err!("unexpected token: {ch:?}")
    } // match
    chars.save_pos();
  } // while
  push!(Tk::EoF);
  tokens
}

pub fn skip_spaces(chars: &mut CharsIter) {
  while let Some(&ch) = chars.peek() {
    if !matches!(ch, ' ' | '\t') {
      break;
    }
    chars.next();
  }
}

pub fn parse_string(chars: &mut CharsIter) -> String {
  let mut string = String::new();
  while let Some(ch) = chars.next() {
    match ch {
      '"' => break,
      '\n' => syntax_err!("unterminated string"),
      '\\' => {
        match chars.next() {
          None => return string,
          Some('"') => string.push('"'),
          Some('\\') => string.push('\\'),
          Some('n') => string.push('\n'),
          Some('r') => string.push('\r'),
          Some('t') => string.push('\t'),
          Some('0') => string.push('\0'),
          Some('e') => string.push('\x1b'),
          Some('u') => {
            if chars.next() != Some('{') {
              syntax_err!("expected '{{' after '\\u'");
            }
            skip_spaces(chars);
            let mut hex = String::new();
            // This shows a warning but if I use a for loop the program crashes...
            #[allow(clippy::while_let_on_iterator)]
            while let Some(&ch) = chars.peek() {
              if !ch.is_ascii_hexdigit() {
                if !matches!(ch, '}' | ' ' | '\t') {
                  syntax_err!("invalid hex in escape sequence");
                }
                break;
              }
              chars.next();
              hex.push(ch);
            }
            if hex.is_empty() {
              syntax_err!("empty escape sequence");
            }
            let code = match u32::from_str_radix(&hex, 16) {
              Ok(code) => code,
              Err(why) => syntax_err!("invalid escape sequence {hex:?} ({why})")
            };
            string.push(char::from_u32(code).unwrap());
            skip_spaces(chars);
            if chars.next() != Some('}') {
              syntax_err!("expected right brace after escape sequence");
            }
          }
          Some(other) => syntax_err!("unknown escape sequence \\{other}")
        }
      },
      _ => string.push(ch)
    }
  }
  string
}
