
// What should it print?
// println(0xff.5)
// - Syntax error: hex numbers cannot have decimals
// - Syntax error: unexpected number. Note: Property names cannot be numbers. Maybe you missed a character before it?
// - 255.5

use crate::types::Value;

use super::Parser;

#[must_use]
pub fn parse_unsigned_number(parser: &mut Parser) -> Value {
  let mut n = String::with_capacity(1);

  if parser.current == '0' {
    parser.next_char();

    if parser.current == 'x' {
      parser.next_char();
      while parser.current.is_ascii_hexdigit() {
        n.push(parser.current);
        parser.next_char();
      }
      let result = u32::from_str_radix(&n, 16)
        .expect("Failed to parse the hex number");
      return Value::UInt32(result)
    }
    
    if parser.current == 'b' {
      parser.next_char();
      while matches!(parser.current, '0' | '1') {
        n.push(parser.current);
        parser.next_char();
      }
      let result = u32::from_str_radix(&n, 2)
        .expect("Failed to parse the binary number");
      return Value::UInt32(result);
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
    let peeked = parser.peek();
    if peeked.is_ascii_digit() {
      //
    }
  }

  let result = n.parse::<u32>()
    .expect("Failed to parse the number");
  Value::UInt32(result)
}
