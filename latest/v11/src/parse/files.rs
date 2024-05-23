
use super::{ parse_const, parse_function, parse_var, Parser };

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
    let mut word = parser.expect_keyword();
    let is_public: bool;

    if matches!(word.as_str(), "pub" | "priv") {
      parser.next_token();
      word = parser.expect_keyword();
      is_public = word == "pub";
    }

    // let item = 
    match word.as_str() {
      "mod" => todo!(),

      "use" => todo!(),

      "typedef" => todo!(),
      "const" => parse_const(&mut parser),
      "var" => parse_var(&mut parser),

      "async" => todo!(),
      "unsafe" => todo!(),
      "fun" => parse_function(&mut parser),

      "class" => todo!(),
      "struct" => todo!(),
      "enum" => todo!(),
      "bitset" => todo!(),

      "trait" => todo!(),
      "impl" => todo!(),

      "macro" => todo!(),

      "testit" => todo!(),
      _ => syntax_err!("unexpected keyword {word:?} here"; parser)
    };
    // parser.set_item(item);
  }
  // parser
}
