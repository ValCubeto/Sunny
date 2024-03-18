use super::{ Parser, parse_function };

/// Parses the items in the provided file content
pub fn parse_file(file_name: &str, data: &str) {
  let mut parser = Parser::new(file_name, data);

  parser.skip_whitespaces();

  println!("curr = {:?}", parser.current);
  match parser.current {
    ch if ch.is_ascii_alphabetic() => {
      let word = parser.parse_ascii_word();
      match word.as_str() {
        "const" => {
          parser.next_token();
          // TODO: maybe we should match `{` or `[` to implement destructuring,
          // and if any alphabetic character is found, then call `parser.parse_word()`
          let ident = parser.expect_word();
          todo!()
        },
        "fun" => {
          parser.skip_whitespaces();
          parse_function(&mut parser);
        }
        _ident => syn_err!("unexpected identifier {word:?} here")
      }
    },
    ch => syn_err!("invalid or unexpected token {ch:?}")
  }
}
