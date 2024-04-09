
// What should it print?
// println(0xff.5)
// - Syntax error: hex numbers cannot have decimals
// - Syntax error: unexpected number. Note: Property names cannot be numbers. Maybe you missed a character before it?
// - 255.5

use crate::types::Value;

use super::Parser;

#[must_use]
pub fn parse_unsigned_number(parser: &mut Parser) -> Number {
  let mut n = String::with_capacity(1);

  if parser.current == '0' {
    parser.next_char();

    if parser.current == 'x' {
      parser.next_char();
      while parser.current.is_ascii_hexdigit() {
        n.push(parser.current);
        parser.next_char();
      }
      return Number::Hex(n)
    }
    
    if parser.current == 'b' {
      parser.next_char();
      while matches!(parser.current, '0' | '1') {
        n.push(parser.current);
        parser.next_char();
      }
      return Number::Bin(n);
    }

    // Nothing happens, continue parsing
  }

  n.push(parser.current);
  while parser.current.is_ascii_digit() {
    n.push(parser.current);
    parser.next_char();
  }

  // How to handle this? The next token can be both a digit and a property name
  // Example: `1.2` or `1.to_string()`
  if parser.current == '.' {
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
    while parser.current.is_ascii_digit() {
      n.push(parser.current);
      parser.next_char();
    }
    return Number::Float(n);
  }

  Number::Uint(n)
}

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
  Exp(String, String)
}
