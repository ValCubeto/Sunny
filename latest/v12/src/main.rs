use std::{iter::Peekable, str::Chars};


fn main() {
  let mut file = include_str!("../files/test.sny")
    .trim_matches(Parser::is_space)
    .to_owned();
  file.push('\0');
  println!("Data: {:?}", file);
  println!();

  #[allow(clippy::needless_borrow)]
  let mut parser = Parser::new(&file);
  parser.next_char();
  parse_expr(&mut parser);
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

pub fn parse_expr(parser: &mut Parser) {
  parser.next_token();
  println!("[idx={}] Parsing expr starting with: {:?}", parser.idx(), parser.current());
  let val = parse_value(parser);
  println!("{:?}", val);
}

pub fn parse_value(parser: &mut Parser) -> Expr {
  parser.next_token();
  match parser.current() {
    '&' => {
      parser.next_char();
      Expr::Ref(parse_value(parser).ptr())
    },
    '*' => {
      parser.next_char();
      Expr::Deref(parse_value(parser).ptr())
    },
    '0'..='9' => {
      let mut n = String::from(parser.current());
      parser.next_char();
      while parser.current().is_ascii_digit() {
        n.push(parser.current());
        parser.next_char();
      }
      Expr::Token(Value::Number(n))
    },
    _ => panic!("unexpected token {:?}", parser.current())
  }
}

#[derive(Debug)]
pub enum Value {
  Number(String)
}

#[derive(Debug)]
pub enum Expr {
  Token(Value),
  Ref(Box<Expr>),
  Deref(Box<Expr>)
}
impl Expr {
  #[inline(always)]
  pub fn ptr(self) -> Box<Expr> {
    Box::new(self)
  }
}
