mod keywords;
mod tokens;
use std::iter::Peekable;
use std::str::Chars;
use keywords::Keyword;
use tokens::Token as Tk;

pub fn tokenize(input: String) -> Vec<Tk> {
  let mut tokens = Vec::new();
  let mut chars = input.chars().peekable();
  while let Some(ch) = chars.next() {
    println!();
    debug_msg!("Current char: {ch:?}");
    match ch {
      ' ' | '\t' => skip_spaces(&mut chars),
      '(' => tokens.push(Tk::LeftParen),
      ')' => tokens.push(Tk::RightParen),
      '{' => tokens.push(Tk::LeftBrace),
      '}' => tokens.push(Tk::RightBrace),
      '[' => tokens.push(Tk::LeftBracket),
      ']' => tokens.push(Tk::RightBracket),
      ',' => tokens.push(Tk::Comma),
      '?' => tokens.push(Tk::Question),
      '!' => match chars.peek() {
        Some('=') => {
          chars.next();
          tokens.push(Tk::NotEqual);
          continue;
        }
        _ => tokens.push(Tk::Bang)
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
        debug_msg!("Skipped: {skipped:?}");
        // Idk how to negate this condition
        if let Some(Tk::NewLine) = tokens.last() {} else {
          tokens.push(Tk::NewLine);
        }
      }
      '<' => match chars.peek() {
        Some('>') => {
          chars.next();
          tokens.push(Tk::Diamond);
        }
        Some('=') => {
          chars.next();
          tokens.push(Tk::LessEqual);
        }
        Some('<') => {
          chars.next();
          if chars.peek() == Some(&'=') {
            chars.next();
            tokens.push(Tk::LeftShiftAssign);
            continue;
          }
          tokens.push(Tk::LeftShift);
        }
        _ => tokens.push(Tk::LeftAngle)
      }
      '>' => match chars.peek() {
        Some('=') => {
          chars.next();
          tokens.push(Tk::GreaterEqual);
        }
        Some('>') => {
          chars.next();
          if chars.peek() == Some(&'=') {
            chars.next();
            tokens.push(Tk::RightShiftAssign);
            continue;
          }
          tokens.push(Tk::RightShift);
        }
        _ => tokens.push(Tk::RightAngle)
      }
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
      }
      ':' => {
        if chars.peek() == Some(&':') {
          chars.next();
          tokens.push(Tk::DoubleColon);
          continue;
        }
        tokens.push(Tk::Colon);
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
        if let Some(Tk::Semicolon) = tokens.last() {} else {
          tokens.push(Tk::Semicolon);
        }
      }
      '+' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::AddAssign);
          continue;
        }
        tokens.push(Tk::Plus);
      }
      '-' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::SubAssign);
          continue;
        }
        tokens.push(Tk::Minus);
      }
      '*' => match chars.peek() {
        Some('=') => {
          chars.next();
          tokens.push(Tk::MulAssign);
        }
        Some('*') => {
          chars.next();
          if chars.peek() == Some(&'=') {
            chars.next();
            tokens.push(Tk::PowAssign);
            continue;
          }
          tokens.push(Tk::DoubleStar);
        }
        _ => tokens.push(Tk::Star)
      }
      '/' => match chars.peek() {
        Some('=') => {
          chars.next();
          tokens.push(Tk::DivAssign);
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
        _ => tokens.push(Tk::Slash)
      }
      '%' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::ModAssign);
          continue;
        }
        tokens.push(Tk::Percent);
      }
      '^' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::XorAssign);
          continue;
        }
        tokens.push(Tk::Xor);
      }
      '&' => match chars.peek() {
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
        }
        // `&=`
        Some(&'=') => {
          chars.next();
          tokens.push(Tk::AndAssign);
        }
        _ => tokens.push(Tk::And)
      }
      '|' => match chars.peek() {
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
        }
        // `|=`
        Some(&'=') => {
          chars.next();
          tokens.push(Tk::OrAssign);
        }
        _ => tokens.push(Tk::Or)
      }
      '=' => match chars.peek() {
        Some('=') => {
          chars.next();
          tokens.push(Tk::DoubleEqual);
          continue;
        }
        Some('>') => {
          chars.next();
          tokens.push(Tk::Arrow);
          continue;
        }
        _ => tokens.push(Tk::Equal)
      },
      '0' => match chars.peek() {
        Some('x') => {
          chars.next();
          tokens.push(Tk::HexNumber(parse_hex(&mut chars)));
        }
        Some('b') => {
          chars.next();
          tokens.push(Tk::BinNumber(parse_bin(&mut chars)));
        }
        Some(&d) if d.is_ascii_digit() => {
          tokens.push(Tk::Int(parse_int(&mut chars, d)));
          chars.next();
        }
        _ => tokens.push(Tk::Int("0".to_owned()))
      }
      d @ '1'..='9' => tokens.push(Tk::Int(parse_int(&mut chars, d))),
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
        debug_msg!("Word: {token:?}");
        tokens.push(token);
      }
      '"' => tokens.push(Tk::String(parse_string(&mut chars))),
      _ => syntax_err!("unexpected token: {ch:?}")
    } // match
  } // while
  tokens
}

pub fn skip_spaces(chars: &mut Peekable<Chars>) {
  while let Some(&ch) = chars.peek() {
    if !matches!(ch, ' ' | '\t') {
      break;
    }
    chars.next();
  }
}

pub fn parse_string(chars: &mut Peekable<Chars>) -> String {
  let mut string = String::new();
  while let Some(ch) = chars.next() {
    match ch {
      '"' => return string,
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
              Err(why) => syntax_err!("invalid escape sequence ({hex:?}): {why}")
            };
            string.push(char::from_u32(code).unwrap());
            skip_spaces(chars);
            if chars.next() != Some('}') {
              syntax_err!("expected `}}` after escape sequence");
            }
          }
          cc => {
            debug!(string);
            debug!(cc);
            syntax_err!("unexpected escape sequence: {cc:?}")
          }
        } // match
      },
      _ => string.push(ch)
    } // match
  } // while
  if chars.peek().is_none() {
    syntax_err!("unterminated string");
  }
  string
}

pub fn parse_int(chars: &mut Peekable<Chars>, digit: char) -> String {
  let mut int = String::from(digit);
  while let Some(&ch) = chars.peek() {
    if !ch.is_ascii_digit() {
      break;
    }
    int.push(ch);
    chars.next();
  }
  int
}

pub fn parse_hex(chars: &mut Peekable<Chars>) -> String {
  let mut hex = String::new();
  while let Some(&ch) = chars.peek() {
    if !ch.is_ascii_hexdigit() {
      break;
    }
    chars.next();
    hex.push(ch);
  }
  hex
}

pub fn parse_bin(chars: &mut Peekable<Chars>) -> String {
  let mut bin = String::new();
  while let Some(&ch) = chars.peek() {
    if !matches!(ch, '0' | '1') {
      break;
    }
    chars.next();
    bin.push(ch);
  }
  bin
}
