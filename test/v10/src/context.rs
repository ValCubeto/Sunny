use std::rc::Rc;
use hashbrown::HashMap;
use crate::values::Value;

pub struct Context {
  data: HashMap<Rc<str>, Value>,
  pub chars: Vec<char>,
  pub char_count: usize,
  pub current: char,
  cursor: usize
}

impl Context {
  pub fn new(string: &str) -> Self {
    let chars: Vec<char> = string.chars().collect();
    Context {
      data: HashMap::new(),
      char_count: chars.len(),
      current: chars[0],
      chars,
      cursor: 0
    }
  }
  pub fn cursor(&self) -> usize {
    self.cursor
  }
  pub fn debug(&self) {
    println!("[{}/{}] {:?}", self.cursor, self.chars.len() - 1, self.current);
  }
  fn advance_cursor(&mut self) {
    self.cursor += 1;
    if self.cursor == self.chars.len() {
      syn_error!("unexpected end of input");
    }
    self.current = self.chars[self.cursor];
  }
  pub fn next_char(&mut self) {
    self.advance_cursor();
    // ignore comments
    if self.current == '/' && self.chars[self.cursor + 1] == '/' {
      while self.current != '\n' {
        self.advance_cursor();
      }
      self.advance_cursor();
    }
    if self.cursor == self.chars.len() - 1 {
      syn_error!("unexpected end of input");
    }
    self.debug();
  }
  /// Skips whitespaces, including new lines.
  pub fn skip_spaces(&mut self) {
    while self.current.is_whitespace() {
      self.advance_cursor();
      // ignore comments
      if self.current == '/' && self.chars[self.cursor + 1] == '/' {
        while self.current != '\n' && self.cursor < self.char_count {
          self.advance_cursor();
        }
        self.advance_cursor();
      }
    }
  }
  /// Skips whitespaces, excluding new lines.
  /// Remember in this language semicolons are optional.
  pub fn skip_spacing(&mut self) {
    while self.current != '\n' && self.current.is_whitespace() {
      self.advance_cursor();
      // ignore comments
      if self.current == '/' && self.chars[self.cursor + 1] == '/' {
        while self.current != '\n' {
          self.advance_cursor();
        }
      }
    }
  }
  pub fn expect_current(&mut self, ch: char) {
    if self.current != ch {
      syn_error!("expected {ch:?}, got {:?}", self.current);
    }
  }
  /// Useful for composed tokens like "=>"
  pub fn expect_char(&mut self, ch: char) {
    self.next_char();
    self.expect_current(ch);
  }
  pub fn expect_token(&mut self, ch: char) {
    self.skip_spaces();
    self.expect_current(ch);
  }
  pub fn expect_word(&mut self) -> String {
    self.skip_spaces();

    // Standard size of a small word, prevents reallocating multiple times.
    // If the word is more than 8 characters long, the capacity will grow
    // to 16.
    let mut word = String::with_capacity(8);

    if !self.current.is_alphabetic() {
      syn_error!("expected a word, found {:?}", self.current);
    }
    word.push(self.current);
    self.next_char();
    while self.current.is_alphanumeric() {
      word.push(self.current);
      self.next_char();
    }
    word
  }
}
