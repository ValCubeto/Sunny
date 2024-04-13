use crate::types::IntermediateValue;

use super::{ parse_value, Parser };

pub fn parse_expr(parser: &mut Parser) -> Expression {
  let left = parse_value(parser);
  println!("Parsed value: {left:?}");
  println!();
  let mut value = Expression::Value(left);
  parser.next_char();
  match parser.current() {
    ';' | ',' => {
      parser.next_char();
      value
    }
    '+' => value
  }
  parse_to_right(parser, value)
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
  match parser.current() {
    ';' | ',' => {
      parser.next_char();
      expr
    }
    '\n' => {
      parser.next_token();
      parse_to_right(parser, expr)
    },
    '+' => {
      let right = parse_value(parser);
      expr
    }
    ch => syntax_err!("unexpected token {ch:?}"; parser)
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
