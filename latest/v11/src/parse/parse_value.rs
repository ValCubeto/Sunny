use crate::types::IntermediateValue;
use super::{ parse_unsigned_number, Parser };

pub fn parse_value(parser: &mut Parser) -> IntermediateValue {
  match parser.current() {
    // '-' => {
    //   let sign = parser.current;
    //   parser.next_token();
    //   if !parser.current.is_ascii_digit() {
    //     syntax_err!("expected digit after sign {sign:?}"; parser);
    //   }
    //   parse_signed_number(parser, sign)
    // },
    // '+' => {
    //   let sign = parser.current;
    //   parser.next_token();
    //   if !parser.current.is_ascii_digit() {
    //     syntax_err!("expected digit after sign {sign:?}"; parser);
    //   }
    //   parse_signed_number(parser, sign)
    // },

    // '\'' => {}
    // '"' => {}
    // 'f' => {}
    // 'r' => {}
    // 'c' => {}
    // '(' => {}
    // '[' => {}
    '0'..='9' => {
      IntermediateValue::Number(parse_unsigned_number(parser))
    }
    // ch if ch.is_alphanumeric() || ch == '_' => {
    //   let word = parser.parse_word();
    //   // What should I do here?
    //   // The variable can be declared later in the file, after the current scope
    // }
    ch => syntax_err!("unexpected token {ch:?}"; parser),
  }
}
