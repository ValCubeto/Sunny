use crate::types::Value;
use super::Parser;

pub fn parse_value(parser: &mut Parser) -> Value {
  match parser.current {
    // '+' => {
    //   let sign = parser.current;
    //   parser.next_char();
    //   if parser.current.is_ascii_digit() {
    //     parse_signed_number(parser, false)
    //   }
    // },
    // '-' => {
    //   let sign = parser.current;
    //   parser.next_char();
    //   if parser.current.is_ascii_digit() {
    //     parse_signed_number(parser, true)
    //   }
    // },
    ch if ch.is_ascii_digit() => {
      parse_unsigned_number(parser)
    }
    // ch if ch.is_alphanumeric() || ch == '_' => {
    //   let word = parser.parse_word();
    //   parser.get_value(word)
    // }
    ch => syn_err!("unexpected token {ch:?}"; parser),
  }
}


