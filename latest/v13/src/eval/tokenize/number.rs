use std::fmt;
use super::CharsIter;

#[derive(Debug, Clone)]
pub enum Number {
  Int(String),
  Float(String, String),
  Hex(String),
  Bin(String),
  Exp(Box<Number>, Box<Number>),
}

impl fmt::Display for Number {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Int(int) => write!(f, "{int}"),
      Self::Float(int, frac) => write!(f, "{int}.{frac}"),
      Self::Hex(hex) => write!(f, "0x{hex}"),
      Self::Bin(bin) => write!(f, "0b{bin}"),
      Self::Exp(lhs, rhs) => write!(f, "exp({lhs}, {rhs})"),
    }
  }
}

pub fn parse_number(chars: &mut CharsIter, digit: char) -> Number {
  let lhs = parse_single_number(chars, digit);
  match chars.peek() {
    Some('e' | 'E') => {
      // input = 5e10
      // this next() skips the 'e' but also skips the last '0' for some reason
      chars.next();
      let d = match chars.next() {
        Some(d) => {
          debug!(d);
          d
        },
        None => syntax_err!("expected exponent"),
      };
      let rhs = parse_single_number(chars, d);
      Number::Exp(Box::new(lhs), Box::new(rhs))
    }
    _ => lhs
  }
}

fn parse_single_number(chars: &mut CharsIter, digit: char) -> Number {
  let mut int = match digit {
    '0' => {
      while chars.peek() == Some(&'0') {
        chars.next();
      }
      String::new()
    },
    digit => String::from(digit)
  };
  while let [Some(a), b] = chars.peek_amount(2) {
    debug_msg!("a = {a:?}; b = {b:?}");
    match (a, b) {
      ('.', Some(d)) if d.is_ascii_digit() => {
        let mut frac = d.to_string();
        chars.next();
        chars.next();
        while let Some(&ch) = chars.peek() {
          if !ch.is_ascii_digit() {
            return Number::Float(int, frac);
          }
          frac.push(ch);
          chars.next();
        }
      }
      ('e' | 'E', _) => break,
      (d, _) if d.is_ascii_alphabetic() => syntax_err!("unexpected character {d:?}"),
      ('_', _) => {
        chars.next();
      }
      (d, _) if d.is_ascii_digit() => {
        int.push(*d);
        chars.next();
      }
      (_, _) => break
    }
  }
  debug!(int);
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

