use std::str::Chars;
use crate::{
  aliases::Id,
  internal_error, syntax_error
};
mod expressions;

#[allow(unused)]
pub struct Context<'a> {
  pub src: Id,
  pub code: &'a str,
  pub chars: Chars<'a>,
  pub current: char,
  pub idx: usize,
  pub line: usize,
  pub column: usize,
}

impl<'a> Context<'a> {
  pub fn new(src: Id, code: &'a str) -> Self {
    if code.is_empty() {
      internal_error!("empty code passed to the context manager");
    }
    let mut chars = code.chars();
    let current = chars.next().unwrap();
    Context {
      src,
      code,
      chars,
      current,
      idx: 1,
      line: 1,
      column: 1
    }
  }

  #[allow(unused)]
  pub fn next_char(&mut self) {
    let ch = match self.chars.next() {
      Some(ch) => ch,
      None => {
        syntax_error!("unexpected end of input"; self);
      }
    }
    // match self.current
    match ch {
      '\n' => {
        self.line += 1;
        self.column = 1;
      }
      '\t' => {
        self.column += 4;
      }
      _ => {
        self.column += 1;
      }
    }
    self.idx += 1;
    self.current = ch;
  }

  #[allow(unused)]
  #[inline]
  pub fn parse_expr(&mut self) {
    expressions::parse_expr(self);
  }
}
