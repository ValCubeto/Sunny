#[macro_use]
mod macros;
#[allow(unused)]
#[macro_use]
mod errors;
#[allow(unused)]
mod parse;
#[allow(unused)]
mod colors;

use crate::parse::Parser;

fn main() {
  let path = "./files/test.sny";

  let mut file = std::fs::read_to_string(path)
    .expect("failed to read file")
    .trim_matches(Parser::is_space)
    .to_owned();
  file.push('\0');

  println!("Data: {:?}", file);
  println!();

  #[allow(clippy::needless_borrow)]
  let mut parser = Parser::new(&path, &file);
  parser.next_char();
  let value = parser.parse_value();
  parser.parse_expr(value);
}
