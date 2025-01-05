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
  let mut lhs = parse_int(chars, digit);
  if let Number::Int(int) = &lhs {
    if let Some('.') = chars.peek() {
      chars.next();
      let digit = match chars.next() {
        Some(d) if d.is_ascii_digit() => d,
        _ => syntax_err!("expected number after decimal point"),
      };
      let frac = parse_int(chars, digit);
      lhs = Number::Float(int.clone(), frac.to_string());
    }
  }
  if let Some('e' | 'E') = chars.peek() {
    chars.next();
    let digit = match chars.next() {
      Some(d) => d,
      None => syntax_err!("expected exponent"),
    };
    let mut rhs = parse_int(chars, digit);
    if let Number::Int(int) = &rhs {
      if let Some('.') = chars.peek() {
        chars.next();
        let digit = match chars.next() {
          Some(d) if d.is_ascii_digit() => d,
          _ => syntax_err!("expected number after decimal point"),
        };
        let frac = parse_int(chars, digit);
        rhs = Number::Float(int.clone(), frac.to_string());
      }
    }
    return Number::Exp(Box::new(lhs), Box::new(rhs));
  }
  lhs
}

/// Parse only `'0'..='9'`, skipping leading zeroes and underscores
fn parse_int(chars: &mut CharsIter, digit: char) -> Number {
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
    match (a, b) {
      ('e' | 'E', _) => break,
      (d, _) if d.is_ascii_alphabetic() => syntax_err!("unexpected character {d:?}"),
      ('_', Some(d)) if d.is_ascii_digit() => {
        chars.next();
      }
      ('_', _) => syntax_err!("expected number after underscore"),
      (d, _) if d.is_ascii_digit() => {
        int.push(*d);
        chars.next();
      }
      (_, _) => break
    }
  }
  Number::Int(int)
}

pub fn parse_hex(chars: &mut CharsIter) -> Number {
  let mut hex = String::new();
  while let [Some(a), b] = chars.peek_amount(2) {
    match (a, b) {
      ('_', Some(d)) if d.is_ascii_hexdigit() => {
        chars.next();
      }
      ('_', _) => syntax_err!("expected digit after underscore"),
      (d, _) if d.is_ascii_hexdigit() => {
        hex.push(*d);
        chars.next();
      }
      (_, _) => break
    }
  }
  Number::Hex(hex)
}

pub fn parse_bin(chars: &mut CharsIter) -> Number {
  let mut bin = String::new();
  while let [Some(a), b] = chars.peek_amount(2) {
    match (a, b) {
      ('_', Some('0' | '1')) => {
        chars.next();
      }
      ('_', _) => syntax_err!("expected digit after underscore"),
      (d @ '0' | d @ '1', _) => {
        bin.push(*d);
        chars.next();
      }
      (_, _) => break
    }
  }
  Number::Bin(bin)
}

