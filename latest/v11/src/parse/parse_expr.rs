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
Expected: Sum(1, Mul(2, 3))

let left = Value(Int(12));
parser.next_token();
match parser.current() {
  '+' => {
    parser.next_token();
    let right = parse_value(parser);
    return Sum(left, right);
  }
  '*' => {
    if parser.peek() == '*' {
      parser.next_token();
      let right = parse_value(parser);
      return Pow(left, right);
    }
    parser.next_token();
    let right = parse_value(parser);
    return Mul(left, right);
  }
  ch => syntax_err!("unexpected token {ch:?}"; parser)
}

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

pub enum Operator {
  /// a + b
  Add,
  /// a - b
  Sub,
  /// a * b
  Mul,
  /// a / b
  Div,

  /// a && b
  And,
  /// a || b
  Or,

  /// a & b
  BinAnd,
  /// a | b
  BinOr,
  /// a ^ b
  BinXor,
}

pub enum Expression {
  Value(IntermediateValue),
  Operation(Operator, Box<Expression>, Box<Expression>),
}
