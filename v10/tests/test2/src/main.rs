use std::*;

fn main() {
  // make the ast parser based on a list of tokens
  let tokens = [
    Token::Number(1),
    Token::Add,
    Token::Number(2),
    Token::Mul,
    Token::Number(3),
  ];
  let expected = Op::Add(
      Op::Number(1).ptr(),
      Op::Mul(
          Op::Number(2).ptr(),
          Op::Number(3).ptr()
      ).ptr()
  );
}

pub enum Token {
  Number(i32),
  Add,
  Mul
}

pub enum Op {
  Number(i32),
  Add(Box<Op>, Box<Op>),
  Mul(Box<Op>, Box<Op>),
}
impl Op {
  /// I hate seeing `Box::new` everywhere
  #[inline(always)]
  pub fn ptr(self) -> Box<Self> {
    Box::new(self)
  }
}
