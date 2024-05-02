
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
    let mut word = parser.parse_ascii_word();
    let is_public: bool;

    if word == "pub" {
      parser.next_token();
      word = parser.parse_ascii_word();
      is_public = true;
    } else if word == "priv" {
      parser.next_token();
      word = parser.parse_ascii_word();
      is_public = false;
    }

    match word.as_str() {
      "const" => {
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
      },
      "var" => todo!(),
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
    }
  }
}
