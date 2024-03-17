use super::Parser;

/// Parses the items in the provided file content
pub fn parse_file(file_name: &str, data: &str) {
  let mut parser = Parser::new(file_name, data);

  parser.skip_whitespaces();

  println!("curr = {:?}", parser.current);
  match parser.current {
    ch if ch.is_alphabetic() => {
      let word = parser.parse_word();
      syn_err!("unexpected identifier {word:?} here")
    },
    ch => syn_err!("invalid or unexpected token {ch:?}")
  }
}
