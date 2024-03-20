
// println(0xff.5)
// - Syntax error: hex numbers cannot have decimals
// - Syntax error: unexpected number. Note: Property names cannot be numbers. Maybe you missed a character before it?
// - 255.5

use crate::types::Value;

use super::Parser;

pub fn parse_unsigned_number(parser: &mut Parser) -> Value {
  let num_type = NumberType::Unsigned;
  if parser.current == '0' {
    parser.next_char();
    // if parser.current == 'x' {}
    // if parser.current == 'b' {}
  }
}

pub fn parse_signed_number(parser: &mut Parser, is_negative: bool) -> Value {
  let num_type = NumberType::Signed;
  if parser.current == '0' {
    parser.next_char();
    // if parser.current == 'x' {}
    // if parser.current == 'b' {}
  }
}

pub enum NumberType {
  Unsigned,
  Signed,
  Float
}
