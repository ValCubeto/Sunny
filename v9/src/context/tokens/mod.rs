pub mod words;
pub mod numbers;

use crate::values::Value;

#[allow(unused)]
#[derive(Debug, Clone)]
#[repr(u8)] // uses isize for some reason
pub enum OperatorKind {
  And, Or,
  Eq, Neq,
  Lt, Gt, LtEq, GtEq,
  Mod,
  Pow,
  Mul, Div,
  Add, Sub,
  Not, Pos, Neg
}

#[derive(Debug)]
pub enum Token {
  Value(Value),
  Op(OperatorKind),
}

#[derive(Debug, Clone)]
pub enum Node {
  Value(Value),
  Op(OperatorKind, Box<Node>, Box<Node>),
}

pub fn precedence(op: &OperatorKind) -> u8 {
  use OperatorKind as O;
  match op {
    O::Not | O::Pos | O::Neg => 0,
    O::Add | O::Sub          => 1,
    O::Mul | O::Div          => 2,
    O::Pow                   => 3,
    O::Mod                   => 4,
    | O::Lt   | O::Gt
    | O::LtEq | O::GtEq      => 5,
    O::Eq  | O::Neq          => 6,
    O::And | O::Or           => 7,
  }
}
