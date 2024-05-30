use std::{iter::Peekable, path::Path, str::Chars};

pub struct Parser<'a> {
  idx: usize,
  current: char,
  data: Peekable<Chars<'a>>,
  path: Box<Path>,
  line: usize,
  column: usize,
}

impl<'a> Parser<'a> {
  pub fn new(path: &'a str, data: &'a str) -> Self {
    let mut this = Parser {
      current: '0',
      idx: 0,
      data: data.chars().peekable(),
      path: Box::from(Path::new(path)),
      line: 1,
      column: 0,
    };
    this.next_char();
    this
  }
  pub fn path(&self) -> &Path {
    &self.path
  }
  pub fn line(&self) -> usize {
    self.line
  }
  pub fn column(&self) -> usize {
    self.column
  }
  pub fn current(&self) -> char {
    self.current
  }
  pub fn idx(&self) -> usize {
    self.idx
  }
  #[inline(always)]
  pub fn is_space(c: char) -> bool {
    c == ' ' ||
    c == '\t' ||
    c == '\n' ||
    c == '\r'
  }
  fn update_cursor_pos(&mut self) {
    //
  }
  pub fn peek(&mut self) -> char {
    *self.data.peek().unwrap()
  }
  fn _next_char(&mut self) {
    self.idx += 1;
    if self.current == '\0' {
      panic!("unexpected end of input");
    }
    self.current = self.data.next().unwrap();
    self.update_cursor_pos();
  }

  #[inline(always)]
  pub fn is_ident(&self) -> bool {
    let ch = self.current;
    !ch.is_ascii_digit() && ch.is_alphabetic() || ch == '_'
  }

  /// Goes to the next character, ignoring the comments
  pub fn next_char(&mut self) {
    self.idx += 1;
    if self.current == '\0' {
      panic!("unexpected end of input");
    }
    self.current = self.data.next().unwrap();
    self.update_cursor_pos();

    while self.current == '/' {
      // Peeks so the current keeps being '/'
      println!("[idx={}] About to check for comment", self.idx);
      let peeked = self.peek();
      if peeked == '/' {
        println!("[idx={}] Inline comment", self.idx);
        self._next_char();
        self._next_char();
        while self.current != '\n' {
          self._next_char();
        }
        println!("[idx={}] End of comment", self.idx);
        continue;
      }
      if peeked == '*' {
        println!("[idx={}] Multiline comment", self.idx);
        self._next_char();
        self._next_char();
        loop {
          if self.current == '*' && self.peek() == '/' {
            self._next_char();
            self._next_char();
            break;
          }
          self._next_char();
        }
        println!("[idx={}] End of comment", self.idx);
        continue;
      }
      break;
    }
    println!("[idx={}] Called next_char and got: {:?}", self.idx, self.current);
  }

  /// Skips the spaces
  pub fn next_token(&mut self) {
    while Self::is_space(self.current) {
      self.next_char();
    }
  }
}