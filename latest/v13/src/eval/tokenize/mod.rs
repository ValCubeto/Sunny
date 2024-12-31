pub mod keywords;
pub mod tokens;
pub mod number;
use std::str::Chars;
use keywords::Keyword;
use number::Number;
use peekmore::{ PeekMore, PeekMoreIterator };
use tokens::{ Operator as Op, Token as Tk };

type CharsIter<'a> = PeekMoreIterator<Chars<'a>>;

pub fn tokenize(input: String) -> Vec<Tk> {
  let mut tokens = Vec::new();
  let mut chars: CharsIter = input.chars().peekmore();
  while let Some(ch) = chars.next() {
    match ch {
      ' ' | '\t' => skip_spaces(&mut chars),
      '(' => tokens.push(Tk::LeftParen),
      ')' => tokens.push(Tk::RightParen),
      '{' => tokens.push(Tk::LeftBrace),
      '}' => tokens.push(Tk::RightBrace),
      '[' => tokens.push(Tk::LeftBracket),
      ']' => tokens.push(Tk::RightBracket),
      ',' => tokens.push(Tk::Comma),
      '?' => tokens.push(Tk::Op(Op::Question)),
      '!' => match chars.peek() {
        Some('=') => {
          chars.next();
          tokens.push(Tk::Op(Op::NotEqual));
          continue;
        }
        _ => tokens.push(Tk::Op(Op::Bang))
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
        if !matches!(tokens.last(), Some(Tk::NewLine)) {
          tokens.push(Tk::NewLine);
        }
      }
      '<' => match chars.peek() {
        Some('>') => {
          chars.next();
          tokens.push(Tk::Op(Op::Diamond));
        }
        Some('=') => {
          chars.next();
          tokens.push(Tk::Op(Op::LessEqual));
        }
        Some('<') => {
          chars.next();
          if chars.peek() == Some(&'=') {
            chars.next();
            tokens.push(Tk::Op(Op::LeftShiftAssign));
            continue;
          }
          tokens.push(Tk::Op(Op::LeftShift));
        }
        _ => tokens.push(Tk::Op(Op::LeftAngle))
      }
      '>' => match chars.peek() {
        Some('=') => {
          chars.next();
          tokens.push(Tk::Op(Op::GreaterEqual));
        }
        Some('>') => {
          chars.next();
          if chars.peek() == Some(&'=') {
            chars.next();
            tokens.push(Tk::Op(Op::RightShiftAssign));
            continue;
          }
          tokens.push(Tk::Op(Op::RightShift));
        }
        _ => tokens.push(Tk::Op(Op::RightAngle))
      }
      '.' => {
        if chars.peek() == Some(&'.') {
          chars.next();
          if chars.peek() == Some(&'.') {
            chars.next();
            tokens.push(Tk::Op(Op::TripleDot));
            continue;
          }
          tokens.push(Tk::Op(Op::DoubleDot));
          continue;
        }
        tokens.push(Tk::Op(Op::Dot));
      }
      ':' => {
        if chars.peek() == Some(&':') {
          chars.next();
          tokens.push(Tk::Op(Op::DoubleColon));
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
          tokens.push(Tk::Op(Op::AddAssign));
          continue;
        }
        tokens.push(Tk::Op(Op::Plus));
      }
      '-' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::Op(Op::SubAssign));
          continue;
        }
        tokens.push(Tk::Op(Op::Minus));
      }
      '*' => match chars.peek() {
        Some('=') => {
          chars.next();
          tokens.push(Tk::Op(Op::MulAssign));
        }
        _ => tokens.push(Tk::Op(Op::Star))
      }
      '/' => match chars.peek() {
        Some('=') => {
          chars.next();
          tokens.push(Tk::Op(Op::DivAssign));
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
        _ => tokens.push(Tk::Op(Op::Slash))
      }
      '%' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::Op(Op::ModAssign));
          continue;
        }
        tokens.push(Tk::Op(Op::Percent));
      }
      '^' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::Op(Op::XorAssign));
          continue;
        }
        tokens.push(Tk::Op(Op::Xor));
      }
      '&' => match chars.peek() {
        // `&&`
        Some(&'&') => {
          chars.next();
          // `&&=`
          if chars.peek() == Some(&'=') {
            chars.next();
            tokens.push(Tk::Op(Op::LogicalAndAssign));
            continue;
          }
          tokens.push(Tk::Op(Op::DoubleAmpersand));
        }
        // `&=`
        Some(&'=') => {
          chars.next();
          tokens.push(Tk::Op(Op::AndAssign));
        }
        _ => tokens.push(Tk::Op(Op::Ampersand))
      }
      '|' => match chars.peek() {
        // `||`
        Some(&'|') => {
          chars.next();
          // `||=`
          if chars.peek() == Some(&'=') {
            chars.next();
            tokens.push(Tk::Op(Op::LogicalOrAssign));
            continue;
          }
          tokens.push(Tk::Op(Op::DoublePipe));
        }
        // `|=`
        Some(&'=') => {
          chars.next();
          tokens.push(Tk::Op(Op::OrAssign));
        }
        _ => tokens.push(Tk::Op(Op::Pipe))
      }
      '=' => match chars.peek() {
        Some('=') => {
          chars.next();
          tokens.push(Tk::Op(Op::DoubleEqual));
          continue;
        }
        Some('>') => {
          chars.next();
          tokens.push(Tk::Arrow);
          continue;
        }
        _ => tokens.push(Tk::Op(Op::Equal))
      },
      '0' => match chars.peek() {
        Some('x') => {
          chars.next();
          tokens.push(Tk::Number(parse_hex(&mut chars)));
        }
        Some('b') => {
          chars.next();
          tokens.push(Tk::Number(parse_bin(&mut chars)));
        }
        Some(&d) if d.is_ascii_digit() => {
          tokens.push(Tk::Number(parse_number(&mut chars, d)));
          // chars.next();
          todo!();
        }
        _ => tokens.push(Tk::Number(Number::Int("0".to_owned())))
      }
      d @ '1'..='9' => tokens.push(Tk::Number(parse_number(&mut chars, d))),
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

pub fn parse_number(chars: &mut CharsIter, digit: char) -> Number {
  let mut int = String::from(digit);
  while let Some(&ch) = chars.peek() {
    match chars.peek_amount(2) {
      [Some('.'), Some(d)] if d.is_ascii_digit() => {
        let mut frac = d.to_string();
        chars.next();
        chars.next();
        while let Some(&ch) = chars.peek() {
          if !ch.is_ascii_digit() {
            return Number::Float(int, frac);
          }
          debug_msg!("frac: {ch:?}");
          frac.push(ch);
          chars.next();
        }
      }
      _ => {}
    }
    if ch == '_' {
      chars.next();
      continue;
    }
    if !ch.is_ascii_digit() {
      break;
    }
    int.push(ch);
    chars.next();
  }
  Number::Int(int)
}

pub fn parse_hex(chars: &mut CharsIter) -> Number {
  let mut hex = String::new();
  while let Some(&ch) = chars.peek() {
    if !ch.is_ascii_hexdigit() {
      break;
    }
    chars.next();
    hex.push(ch);
  }
  Number::Hex(hex)
}

pub fn parse_bin(chars: &mut CharsIter) -> Number {
  let mut bin = String::new();
  while let Some(&ch) = chars.peek() {
    if !matches!(ch, '0' | '1') {
      break;
    }
    chars.next();
    bin.push(ch);
  }
  Number::Bin(bin)
}
