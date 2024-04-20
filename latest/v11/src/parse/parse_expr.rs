use crate::types::IntermediateValue;

use super::{ parse_value, Parser };

pub fn parse_expr(parser: &mut Parser) -> Expression {
  let left = parse_value(parser);
  println!("Parsed value: {left:?}");
  println!();
  parser.skip_spaces();
  let line_broken = parser.current() == '\n';
  parse_to_right(parser, Expression::Value(left))
}

/*

`1 + 2 * 3`
Expected: E(Sum, 1, E(Mul, 2, 3))

let left = Value(Int(12));
parser.next_token();
match parser.current() {
  '+' => {
    parser.next_token();
    let right = parse_value(parser);
    return Expr(Sum, left, right);
  }
  '*' => {
    if parser.peek() == '*' {
      parser.next_token();
      let right = parse_value(parser);
      return Expr(Pow, left, right);
    }
    parser.next_token();
    let right = parse_value(parser);
    return Expr(Mul, left, right);
  }
  _ => left
}


let left: Val | Expr = parse()
*/

fn parse_to_right(parser: &mut Parser, expr: Expression) -> Expression {
  let right_op = match parser.current() {
    '?' => {
      return Expression::Operation(Operator::Try, Box::new(expr))
    }
    '+' => BinOperator::Add,
    '-' => BinOperator::Sub,
    '/' => BinOperator::Div,
    '^' => BinOperator::BitXor,
    '*' => {
      if parser.peek() == '*' {
        parser.next_token();
        BinOperator::Pow
      } else {
        BinOperator::Mul
      }
    },
    '|' => {
      if parser.peek() == '|' {
        parser.next_token();
        BinOperator::Or
      } else {
        BinOperator::BitXor
      }
    }
    '&' => {
      if parser.peek() == '&' {
        parser.next_token();
        BinOperator::Or
      } else {
        BinOperator::BitXor
      }
    }
    '>' => {
      if parser.peek() == '=' {
        parser.next_token();
        BinOperator::GreaterThanOrEq
      } else {
        BinOperator::GreaterThan
      }
    }
    // '<' => {
    //   if
    //     matches!(
    //       left,
    //       | Expression::Value(IntermediateValue::Identifier(_))
    //     )
    //   {
    //     //
    //   }
    //   let op = BinOperator::LessThan;
    // }
    _ => return expr
  };
  parser.next_char();
  parser.next_token();
  println!("curr = {:?}", parser.current());
  let right = match expr {
    Expression::Value(_) => {
      let right = Expression::Value(parse_value(parser));
      Expression::BinOperation(right_op, Box::new(expr), Box::new(right))
    }
    Expression::BinOperation(left_op, left, right) => {
      let third = Expression::Value(parse_value(parser));

      if left_op.precedence() < right_op.precedence() {
        Expression::BinOperation(
          right_op,
          Box::new(Expression::BinOperation(left_op, left, right)),
          Box::new(third)
        )
      } else {
        Expression::BinOperation(
          left_op,
          left,
          Box::new(Expression::BinOperation(right_op, right, Box::new(third)))
        )
      }
    }
    _ => unimplemented!("idk what should i put here")
  };
  parser.next_token();
  parse_to_right(parser, right)
}

/// Operators that take two values
#[derive(Debug)]
pub enum BinOperator {
  /// `a + b`
  Add,
  /// `a - b`
  Sub,
  /// `a * b`
  Mul,
  /// `a / b`
  Div,
  /// `a ** b`
  Pow,
  /// `a % b`
  Mod,

  /// `a && b`
  And,
  /// `a || b`
  Or,

  /// a > b
  GreaterThan,
  /// `a >= b`
  GreaterThanOrEq,
  /// `a < b`
  LessThan,
  /// `a <= b`
  LessThanOrEq,

  /// `a == b`
  Equal,
  /// `a != b`
  NotEqual,

  /// `a & b`
  BitAnd,
  /// `a | b`
  BitOr,
  /// `a ^ b`
  BitXor,

  /// `a.b`
  GetProp,
  /// `a::b`
  GetItem,
}

#[derive(Debug)]
pub enum Operator {
  /// `-a`
  Negate,

  /// `!a`
  Not,

  /// `a?`
  Try,

  /// `&a`
  Ref,
  /// `*a`
  Deref
}

pub trait Precedence {
  fn precedence(&self) -> u8;
}

impl Precedence for BinOperator {
  fn precedence(&self) -> u8 {
    use BinOperator as O;
    match self {
      // Less precedence than `Operator`
      // so `a.b?`, `!a.b`, `&a::b`, etc. works correctly.
      O::GetProp => 0,
      O::GetItem => 0,

      // Greater precedence than `Operator`
      // so `*a ** b` works correctly
      O::Pow => 2,
      
      O::Mul => 3,
      O::Div => 3,
      O::Mod => 3,
      
      O::Add => 4,
      O::Sub => 4,

      O::BitAnd => 5,
      O::BitOr => 5,
      O::BitXor => 5,

      O::And => 6,
      O::Or => 6,
      O::GreaterThan => 6,
      O::GreaterThanOrEq => 6,
      O::LessThan => 6,
      O::LessThanOrEq => 6,

      O::Equal => 7,
      O::NotEqual => 7,
    }
  }
}

impl Precedence for Operator {
  fn precedence(&self) -> u8 {
    1
  }
}

pub enum TriOperator {
  /// `a then b else c`
  ThenElse,

  /// `a |> b: c`
  Pipe
}

#[derive(Debug)]
pub enum Expression {
  Value(IntermediateValue),
  // TriOperation(TriOperator, Box<Expression>, Box<Expression>, Box<Expression>),
  BinOperation(BinOperator, Box<Expression>, Box<Expression>),
  Operation(Operator, Box<Expression>)
}

// b if a else c
// a then b else c

/*
a then { b } else { c }
*/
