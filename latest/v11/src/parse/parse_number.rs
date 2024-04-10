
// What should it print?
// println(0xff.5)
// - Syntax error: hex numbers cannot have decimals
// - Syntax error: unexpected number. Note: Property names cannot be numbers. Maybe you missed a character before it?
// - 255.5

use super::Parser;

#[must_use]
pub fn parse_unsigned_number(parser: &mut Parser) -> Number {
  let mut n = String::with_capacity(1);

  if parser.current() == '0' {
    parser.next_char();

    if matches!(parser.current(), 'x' | 'X') {
      parser.next_char();
      loop {
        if parser.current().is_ascii_hexdigit() {
          n.push(parser.current());
          parser.next_char();
          continue;
        }
        if parser.current() == '_' {
          continue;
        }
        break;
      }
      return Number::Hex(n)
    }
    
    if matches!(parser.current(), 'b' | 'B') {
      parser.next_char();
      loop {
        if matches!(parser.current(), '0' | '1') {
          n.push(parser.current());
          parser.next_char();
          continue;
        }
        if parser.current() == '_' {
          continue;
        }
        if matches!(parser.current(), '2'..='9') {
          syntax_err!("unexpected decimal in binary number"; parser);
        }
        break;
      }
      return Number::Bin(n);
    }

    // Nothing happens, continue parsing
  }

  n.push(parser.current());
  parser.next_char();
  let result = parse_decimal(parser, n);
  if !matches!(parser.current(), 'e' | 'E') {
    return result;
  }
  parser.next_char();
  // TODO: add signed exponents
  let exponent = parse_decimal(parser, String::new());
  Number::Exp(Box::new(result), Box::new(exponent))
}

fn parse_decimal(parser: &mut Parser, mut n: String) -> Number {
  loop {
    if parser.current().is_ascii_digit() {
      n.push(parser.current());
      parser.next_char();
      continue;
    }
    if parser.current() == '_' {
      continue;
    }
    break;
  }

  if parser.current() == '.' {
    // Peeked because the next char can be a property name
    // For example, `1.to_string`
    // So if it is not a float, we let the current char be the dot
    let peeked = parser.peek();
    if !peeked.is_ascii_digit() {
      return Number::Uint(n);
    }
    n.push('.');
    n.push(peeked);
    // Skip the dot and the peeked
    parser.next_char();
    parser.next_char();
    loop {
      if parser.current().is_ascii_digit() {
        n.push(parser.current());
        parser.next_char();
      }
      if parser.current() == '_' {
        continue;
      }
      break;
    }
    return Number::Float(n);
  }
  
  Number::Uint(n)
}

#[derive(Debug)]
pub enum Number {
  /// 0000_0001 -> 1
  Bin(String),
  // ff -> 255
  Hex(String),
  // -230
  Int(String),
  // 5
  Uint(String),
  // 3.1415
  Float(String),
  /// 2e10 -> 20_000_000_000
  /// Of course these numbers won't be Bin, Hex, or another Exp
  Exp(Box<Number>, Box<Number>)
}
