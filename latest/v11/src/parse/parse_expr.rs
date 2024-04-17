use crate::types::IntermediateValue;

use super::{ parse_value, Parser };

pub fn parse_expr(parser: &mut Parser) -> Expression {
  let left = parse_value(parser);
  println!("Parsed value: {left:?}");
  println!();
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

fn parse_to_right(parser: &mut Parser, left: Expression) -> Expression {
  match parser.current() {
    ';' | ',' => {
      parser.next_char();
      left
    }
    '\n' => {
      parser.next_token();
      parse_to_right(parser, left)
    },
    '+' => {
      let right = parse_value(parser);
      expr
    }
    _ => left
  }
}

/// Operators that take two values
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

impl BinOperator {
  pub fn precedence(&self) -> u8 {
    use BinOperator as O;
    match self {
      O::GetProp => 0,
      O::GetItem => 0,

      O::Pow => 1,
      
      O::Mul => 2,
      O::Div => 2,
      O::Mod => 2,
      
      O::Add => 3,
      O::Sub => 3,

      O::BitAnd => 4,
      O::BitOr => 4,
      O::BitXor => 4,

      O::And => 5,
      O::Or => 5,
      O::GreaterThan => 5,
      O::GreaterThanOrEq => 5,
      O::LessThan => 5,
      O::LessThanOrEq => 5,

      O::Equal => 6,
      O::NotEqual => 6,
    }
  }
}

/// Precedence: 1
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

pub enum TriOperator {
  /// `a then b else c`
  ThenElse,

  /// `a |> b: c`
  Pipe
}

pub enum Expression {
  Value(IntermediateValue),
  TriOperation(TriOperator, Box<Expression>, Box<Expression>, Box<Expression>),
  BinOperation(BinOperator, Box<Expression>, Box<Expression>),
  Operation(Operator, Box<Expression>)
}

// b if a else c
// a then b else c

/*
a then { b } else { c }
*/
