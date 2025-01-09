use std::fmt;
use super::CharsIter;

#[derive(Debug, Clone)]
pub enum Number {
  /// `'0'..='9'`
  Int(String),
  /// `'0'..='9'` `'.'` `'0'..='9'`
  Float(String, String),
  /// `'0x'` `(Int | 'a'..='f' | 'A'..='F')`
  Hex(String),
  /// `'0b'` `('0' | '1')`
  Bin(String),
  /// `Float` `e` `Float`
  Exp(Box<Number>, Box<Number>),
}

impl fmt::Display for Number {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Int(int) => write!(f, "{int}"),
      Self::Float(int, frac) => write!(f, "{int}.{frac}"),
      Self::Hex(hex) => write!(f, "0x{hex}"),
      Self::Bin(bin) => write!(f, "0b{bin}"),
      Self::Exp(lhs, rhs) => write!(f, "{lhs}e{rhs}"),
    }
  }
}

pub fn parse_number(chars: &mut CharsIter, digit: char) -> (Number, usize) {
  let (lhs, mut len) = parse_int(chars, digit);
  let mut lhs = Number::Int(lhs);
  if let Number::Int(int) = &lhs {
    if let Some('.') = chars.peek() {
      chars.next();
      let digit = match chars.next() {
        Some(d) if d.is_ascii_digit() => d,
        _ => syntax_err!("expected number after decimal point"),
      };
      let (frac, len2) = parse_int(chars, digit);
      len += len2 + 1;
      lhs = Number::Float(int.clone(), frac.to_string());
    }
  }
  if let Some('e' | 'E') = chars.peek() {
    chars.next();
    let digit = match chars.next() {
      Some(d) => d,
      None => syntax_err!("expected exponent"),
    };
    let (rhs, len2) = parse_int(chars, digit);
    let mut rhs = Number::Int(rhs);
    len += len2;
    if let Number::Int(int) = &rhs {
      if let Some('.') = chars.peek() {
        chars.next();
        let digit = match chars.next() {
          Some(d) if d.is_ascii_digit() => d,
          _ => syntax_err!("expected number after decimal point"),
        };
        let (frac, len2) = parse_int(chars, digit);
        len += len2 + 1;
        rhs = Number::Float(int.clone(), frac);
      }
    }
    return (Number::Exp(Box::new(lhs), Box::new(rhs)), len);
  }
  (lhs, len)
}

/// Parse only `'0'..='9'`, skipping leading zeroes and underscores
fn parse_int(chars: &mut CharsIter, digit: char) -> (String, usize) {
  let mut len = 1;
  let mut int = match digit {
    '0' => {
      while chars.peek() == Some('0') {
        len += 1;
        chars.next();
      }
      String::new()
    },
    digit => String::from(digit)
  };
  while let [Some(a), b] = chars.peek_amount(2) {
    match (a, b) {
      ('e' | 'E', _) => {
        len += 1;
        break
      },
      (d, _) if d.is_ascii_alphabetic() => {
        let d = chars.next().unwrap();
        syntax_err!("unexpected character {d:?}");
      },
      ('_', Some(d)) if d.is_ascii_digit() => {
        len += 1;
        chars.next();
      }
      ('_', _) => {
        chars.next();
        syntax_err!("expected number after underscore")
      },
      (d, _) if d.is_ascii_digit() => {
        int.push(*d);
        len += 1;
        chars.next();
      }
      (_, _) => break
    }
  }
  (int, len)
}

pub fn parse_hex(chars: &mut CharsIter) -> (Number, usize) {
  let mut hex = String::new();
  let mut len = 2;
  match chars.peek() {
    Some(d) if d.is_ascii_hexdigit() => {
      chars.next();
      hex.push(d);
      len += 1;
    }
    _ => syntax_err!("expected hexadecimal digit"),
  }
  while let [Some(a), b] = chars.peek_amount(2) {
    match (a, b) {
      ('_', Some(d)) if d.is_ascii_hexdigit() => {
        len += 1;
        chars.next();
      }
      ('_', _) => {
        chars.next();
        syntax_err!("expected digit after underscore")
      },
      (d, _) if d.is_ascii_hexdigit() => {
        len += 1;
        hex.push(*d);
        chars.next();
      }
      (_, _) => break
    }
  }
  (Number::Hex(hex), len)
}

pub fn parse_bin(chars: &mut CharsIter) -> (Number, usize) {
  let mut bin = String::new();
  let mut len = 2;
  match chars.peek() {
    Some(d @ ('0' | '1')) => {
      chars.next();
      bin.push(d);
      len += 1;
    }
    _ => syntax_err!("expected digit"),
  }
  while let [Some(a), b] = chars.peek_amount(2) {
    match (a, b) {
      ('_', Some('0' | '1')) => {
        len += 1;
        chars.next();
      }
      ('_', _) => {
        chars.next();
        syntax_err!("expected binary digit after underscore");
      },
      (&d @ '0' | &d @ '1', _) => {
        len += 1;
        bin.push(d);
        chars.next();
      }
      (c, _) if c.is_ascii_alphanumeric() => {
        syntax_err!("expected binary digit");
      }
      (_, _) => break
    }
  }
  (Number::Bin(bin), len)
}
