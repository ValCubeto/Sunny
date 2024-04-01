use crate::parse::parse_expr;

use super::{ Parser, parse_function };

/// Parses the items in the provided file content
pub fn parse_file(file_name: &str, data: &str) {
  let mut parser = Parser::new(file_name, data);

  parser.skip_whitespaces();
  match parser.current {
    ch if ch.is_ascii_alphabetic() => {
      let word = parser.parse_ascii_word();
      match word.as_str() {
        "const" => {
          parser.next_token();
          // HINT: maybe we should match `{` or `[` to implement destructuring,
          // and if any alphabetic character is found, then call `parser.parse_word()`
          let ident = parser.expect_word();
          println!("const {ident:?}");
          // TODO: expect ':' and a type
          // TODO: expect '=' and an expression
          parser.next_token();
          if parser.current == ':' {
            todo!("const typing")
          }
          parser.expect('=');
          parser.next_token();
          parse_expr(&mut parser);
          todo!()
        },
        "fun" => {
          parser.skip_whitespaces();
          parse_function(&mut parser);
        }
        _ident => syntax_err!("unexpected identifier {word:?} here"; parser)
      }
    },
    _ch => syntax_err!("invalid or unexpected token {:?}", parser.current; parser)
  }
}
