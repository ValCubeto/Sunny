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

  let ast = parse_tokens(&tokens);

  assert_eq!(ast, expected);
}

pub enum Token {
  Number(i32),
  Add,
  Mul
}

#[derive(PartialEq, Eq, Debug)]
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

pub fn parse_tokens(tokens: &[Token]) -> Op {
  assert!(!tokens.is_empty());
  let mut iter = tokens.iter();

  let mut result = Op::Number(0);
  
  if let &Token::Number(left) = iter.next().unwrap() {
    if tokens.len() == 1 {
      return Op::Number(left);
    }
    match *iter.next().unwrap() {
      Token::Number(_) => panic!("expected op"),
      Token::Add => {
        if let &Token::Number(right) = iter.next().expect("unexpected end of input") {
          result = Op::Add(Op::Number(left).ptr(), Op::Number(right).ptr());
        } else {
          panic!("expected num")
        }
      }
      Token::Mul => {
        if let &Token::Number(right) = iter.next().expect("unexpected end of input") {
          result = Op::Mul(Op::Number(left).ptr(), Op::Number(right).ptr());
        } else {
          panic!("expected num")
        }
      }
      _ => unimplemented!()
    }
  }
  result
}










pub enum Operator {
  Number,
  Add,
  Mul,
}

pub struct Operation(Operator, Box<Operation>, Box<Operation>);

fn t() {
  let _ = Operation(
    Box::new(Operator::Add),
    Operation(Operator::Number, )
  );
}















