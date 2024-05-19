
use super::{ Parser, parse_function };

/// Parses the items in the provided file content
pub fn parse_file(file_name: &str, data: &str) {
  let mut parser = Parser::new(file_name, data);

  loop {
    parser.skip_whitespaces();
    if parser.current() == '#' {
      parser.next_char();
      if parser.current() == '!' {
        parser.next_char();
        todo!();
      }
      parser.expect('[');
      todo!();
    }
    if !parser.current().is_ascii_alphabetic() {
      syntax_err!("invalid or unexpected token {:?}", parser.current(); parser);
    }
    let mut word = parser.expect_keyword();
    let is_public: bool;

    if matches!(word.as_str(), "pub" | "priv") {
      parser.next_token();
      word = parser.expect_keyword();
      is_public = word == "pub";
    }

    // let item = 
    match word.as_str() {
      "const" => parse_const(&mut parser),
      // "var" => parse_var(&mut parser),
      "fun" => {
        parser.skip_whitespaces();
        parse_function(&mut parser);
        todo!();
      }
      "struct" => todo!(),
      "enum" => todo!(),
      "flagset" => todo!(),
      "class" => todo!(),
      "trait" => todo!(),
      "mod" => todo!(),
      "if" => todo!(),
      "pub" => todo!(),
      "priv" => todo!(),
      "impl" => todo!(),
      "typedef" => todo!(),
      "use" => todo!(),
      "unsafe" => todo!(),
      "async" => todo!(),
      "test" => todo!(),
      _ => syntax_err!("unexpected token {word:?} here"; parser)
    };
    // parser.set_item(item);
  }
  // parser
}
