pub mod statements;
pub mod expressions;
pub mod tokens;

use std::{
  str::Chars,
  iter::Peekable
};
use crate::{
  aliases::Id,
  syntax_error,
  debug
};

#[allow(unused)]
pub struct Context<'a> {
  pub src: Id,
  pub code: &'a str,
  pub chars: Peekable<Chars<'a>>,
  pub current: char,
  pub idx: usize,
  pub line: usize,
  pub column: usize,
}

impl<'a> Context<'a> {
  pub fn new(src: Id, code: &'a str) -> Self {
    let mut chars = code.chars().peekable();
    let current = chars.next().unwrap();
    Context {
      src,
      code,
      chars,
      current,
      idx: 1,
      line: 1,
      column: 0 // usually '{' is the first character
    }
  }

  #[allow(unused)]
  pub fn debug(&self) {
    debug!("chars[{}]({}:{}:{}) = {:?}", self.idx, self.src, self.line, self.column, self.current);
  }

  #[allow(unused)]
  pub fn next_char(&mut self) {
    let ch = match self.chars.next() {
      Some(ch) => ch,
      None => syntax_error!("unexpected end of input"; self)
    };
    match self.current {
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
    debug!("               called next_char(). Current: {:?}", self.current);
  }

  pub fn parse_block(&mut self) {
    debug!("parsing block: {:?}", self.code);
    if self.current != '{' {
      syntax_error!("expected '{{', got {:?}", self.current; self);
    }
    self.next_char();
    todo!();
    // while self.current != '}' {
    //   // self.parse_statement();
    //   // self.next_token();
    // }
  }
}
