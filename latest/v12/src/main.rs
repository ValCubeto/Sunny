use std::{iter::Peekable, str::Chars};


fn main() {
  let mut file = include_str!("../files/test.sny")
    .trim_matches(Parser::is_space)
    .to_owned();
  file.push('\0');
  println!("{:?}", file);

  #[allow(clippy::needless_borrow)]
  let parser = Parser::new(&file);
}

pub struct Parser<'a> {
  idx: usize,
  current: char,
  data: Peekable<Chars<'a>>
}
impl<'a> Parser<'a> {
  pub fn new(data: &'a str) -> Self {
    Parser {
      data: data.chars().peekable(),
      idx: 0,
      current: '\0'
    }
  }
  #[inline(always)]
  pub fn is_space(c: char) -> bool {
    c == ' ' ||
    c == '\t' ||
    c == '\n' ||
    c == '\r'
  }
}
