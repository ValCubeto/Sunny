use crate::types::IntermediateValue;

use super::{ parse_value, Parser };

pub fn parse_expr(parser: &mut Parser) -> Expression {
  let left = parse_value(parser);
  println!("Parsed value: {left:?}");
  println!();
  let mut value = Expression::Value(left);
  parse_to_right(parser, value)
}

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
      match expr {
        Expression::Value()
      }
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
