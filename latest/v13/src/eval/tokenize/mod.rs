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
      }
      '<' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::LessEqual);
          continue;
        }
        tokens.push(Tk::LeftAngle);
      }
      '>' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::GreaterEqual);
          continue;
        }
        tokens.push(Tk::RightAngle);
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
        }
        tokens.push(Tk::Semicolon);
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
      '*' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::MulAssign);
          continue;
        }
        tokens.push(Tk::Star);
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
          // tokens.push(Tk::Comment);
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
            eprintln!("Unclosed comment!!");
            std::process::exit(1);
          }
          // tokens.push(Tk::Comment);
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
      }
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
      }
      '=' => {
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Tk::DoubleEqual);
          continue;
        }
        tokens.push(Tk::Equal);
      }
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
          Some('e') => string.push_str("\x1b["),
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
