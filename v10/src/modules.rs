use std::{str::Chars, iter::Enumerate};


#[allow(unused_labels, clippy::while_let_on_iterator)]
pub fn parse_module(code: &str) {
  let mut parser = ModuleParser::new(code);
  'a: while let Some((i, ch)) = parser.try_next_char() {
    if 
    println!("({i}, {ch:?})");
  }
}

pub struct ModuleParser<'a> {
  chars: Enumerate<Chars<'a>>
}
impl<'a> ModuleParser<'a> {
  #[inline]
  pub fn new(code: &'a str) -> ModuleParser<'a> {
    ModuleParser {
      chars: code.chars().enumerate()
    }
  }
  #[inline]
  pub fn try_next_char(&mut self) -> Option<(usize, char)> {
    self.chars.next()
  }

  #[inline]
  pub fn next_char(&mut self) -> (usize, char) {
    match self.chars.next() {
      None => syn_error!("unexpected end of input"),
      Some(pair) => pair
    }
  }
}
