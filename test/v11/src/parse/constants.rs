use super::Parser;

pub fn parse_const(parser: &mut Parser) {
  parser.next_token();
  // HINT: maybe we should match `{` or `[` to implement destructuring,
  // and if any alphabetic character is found, then call `parser.parse_word()`
  let ident = parser.expect_word();

  parser.next_token();
  // parser.expect(':');

  parser.expect('=');
  parser.next_token();
  let value = parse_expr(&mut parser);
  println!("{:#?}", value);
  todo!();
}
