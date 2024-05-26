use std::{iter::Peekable, str::Chars};


fn main() {
  let mut file = include_str!("../files/test.sny")
    .trim_matches(Parser::is_space)
    .to_owned();
  file.push('\0');
  println!("{:?}", file);

  #[allow(clippy::needless_borrow)]
  let mut parser = Parser::new(&file);
  parser.next_char();
  println!("{}: {:?}", parser.idx(), parser.current());
  // parse_expr(&mut parser);
}

#[allow(unused)]
pub struct Parser<'a> {
  idx: usize,
  current: char,
  data: Peekable<Chars<'a>>
}
impl<'a> Parser<'a> {
  pub fn new(data: &'a str) -> Self {
    let mut this = Parser {
      current: '0',
      idx: 0,
      data: data.chars().peekable(),
    };
    this.next_char();
    // this.update_cursor_pos();
    this
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
  pub fn next_char(&mut self) {
    self.idx += 1;
    if self.current == '\0' {
      panic!("unexpected end of input");
    }
    self.current = self.data.next().unwrap();
    self.update_cursor_pos();

    while self.current == '/' {
      // Peeks so the current keeps being '/'
      println!("maybe comment: {:?}", self.current);
      let peeked = self.peek();
      if peeked == '/' {
        println!("inline comment");
        self._next_char();
        self._next_char();
        while self.current != '\n' {
          self._next_char();
        }
        println!("about to continue with curr = {:?}", self.current);
        continue;
      }
      if peeked == '*' {
        println!("block comment");
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
        continue;
      }
      break;
    }
    println!("    {}: {:?}", self.idx, self.current);
  }
}
