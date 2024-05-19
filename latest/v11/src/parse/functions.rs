use super::Parser;

pub fn parse_function(parser: &mut Parser) {
  let name = parser.expect_ident();
  parser.skip_whitespaces();
  if parser.current() != '<' {
    parser.next_token();
    todo!();
  }
  parser.expect('(');
  parser.skip_whitespaces();
  let mut args = Vec::new();
  if parser.current() != ')' {
    loop {
      let arg = parser.expect_ident();
      args.push(arg);
      parser.skip_whitespaces();
      if parser.current() == ')' {
        break;
      }
      parser.expect(',');
      parser.skip_whitespaces();
    }
  }
}
