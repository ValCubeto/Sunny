pub mod words;
pub mod numbers;

use crate::values::Value;

#[allow(unused)]
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Operator {
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
  Op(Operator)
}

#[derive(Debug, Clone)]
pub enum Node {
  Value(Value),
  Op(Operator, Box<Node>, Box<Node>),
}

pub fn precedence(op: &Operator) -> u8 {
  use Operator as O;
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

pub fn build_ast(tokens: Vec<Token>) -> Node {
  let mut operators: Vec<Operator> = Vec::new();
  let mut output: Vec<Node> = Vec::new();

  for token in tokens {
    match token {
      Token::Value(value) => output.push(Node::Value(value)),
      Token::Op(ref op) => {
        while let Some(stack_op) = operators.last() {
          if precedence(op) > precedence(stack_op) {
            break;
          }
          let right = output.pop().unwrap();
          let left = output.pop().unwrap();
          let node = Node::Op(stack_op.clone(), Box::new(left), Box::new(right));
          output.push(node);
          operators.pop();
        }
        operators.push(op.clone());
      }
    }
  }

  while let Some(op) = operators.pop() {
    let right = output.pop().unwrap();
    let left = output.pop().unwrap();
    let node = Node::Op(op, Box::new(left), Box::new(right));
    output.push(node);
  }

  output.pop().unwrap()
}
