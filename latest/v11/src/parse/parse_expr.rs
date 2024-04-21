use crate::types::IntermediateValue;

use super::{ parse_value, Parser };

pub fn parse_expr(parser: &mut Parser) -> Expr {
  let left = parse_value(parser);
  println!("Parsed value: {left:?}");
  println!();
  parser.next_token();
  let line_broken = parser.current() == '\n';
  parse_to_right(parser, Expr::Value(left))
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

fn parse_to_right(parser: &mut Parser, expr: Expr) -> Expr {
  let right_op = match parser.current() {
    '.' => {
      // if parser.peek() == '.' {}
      BinOp::GetProp
    }
    ':' => {
      parser.expect(':');
      BinOp::GetItem
    }
    '?' => {
      parser.next_char();
      parser.next_token();
      let expr = match expr {
        Expr::BinOperation(op, left, right) => {
          if op.precedence() < Op::Try.precedence() {
            Expr::Operation(Op::Try, Box::new(Expr::BinOperation(op, left, right)))
          } else {
            Expr::BinOperation(op, left, Box::new(Expr::Operation(Op::Try, right)))
          }
        }
        | Expr::Operation(_, _)
        | Expr::Value(_) => {
            Expr::Operation(Op::Try, Box::new(expr))
          }
        // _ => todo!()
      };
      return parse_to_right(parser, expr);
    }
    '+' => BinOp::Add,
    '-' => BinOp::Sub,
    '/' => BinOp::Div,
    '^' => BinOp::BitXor,
    '*' => {
      if parser.peek() == '*' {
        parser.next_token();
        BinOp::Pow
      } else {
        BinOp::Mul
      }
    },
    '|' => {
      if parser.peek() == '|' {
        parser.next_token();
        BinOp::Or
      } else {
        BinOp::BitXor
      }
    }
    '&' => {
      if parser.peek() == '&' {
        parser.next_token();
        BinOp::Or
      } else {
        BinOp::BitXor
      }
    }
    '>' => {
      if parser.peek() == '=' {
        parser.next_token();
        BinOp::GreaterThanOrEq
      } else {
        BinOp::GreaterThan
      }
    }
    // '<' => {
    //   if
    //     matches!(
    //       left,
    //       | Expr::Value(IntermediateValue::Identifier(_))
    //     )
    //   {
    //     //
    //   }
    //   let op = BinOp::LessThan;
    // }
    ch => {
      println!("ch = {ch:?}");
      return expr
    }
  };
  parser.next_char();
  parser.next_token();
  println!("curr = {:?}", parser.current());
  let right = match expr {
    Expr::Value(_) => {
      let right = Expr::Value(parse_value(parser));
      Expr::BinOperation(right_op, Box::new(expr), Box::new(right))
    }
    Expr::Operation(_, _) => {
      let right = Expr::Value(parse_value(parser));
      Expr::BinOperation(right_op, Box::new(expr), Box::new(right))
    }
    Expr::BinOperation(left_op, left, right) => {
      let third = Expr::Value(parse_value(parser));

      if left_op.precedence() < right_op.precedence() {
        Expr::BinOperation(
          left_op,
          left,
          Box::new(Expr::BinOperation(right_op, right, Box::new(third))),
        )
      } else {
        Expr::BinOperation(
          right_op,
          Box::new(Expr::BinOperation(left_op, left, right)),
          Box::new(third),
        )
      }
    }
    _ => unimplemented!("idk what should i put here")
  };
  parser.next_token();
  parse_to_right(parser, right)
}

/// Ops that take two values
#[derive(Debug)]
pub enum BinOp {
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
pub enum Op {
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

impl Precedence for BinOp {
  fn precedence(&self) -> u8 {
    use BinOp as O;
    match self {
      // Less precedence than `Op`
      // so `a.b?`, `!a.b`, `&a::b`, etc. works correctly.
      O::GetItem => 0,
      O::GetProp => 1,

      // Greater precedence than `Op`
      // so `*a ** b` works correctly
      O::Pow => 3,
      
      O::Mul => 4,
      O::Div => 4,
      O::Mod => 4,
      
      O::Add => 5,
      O::Sub => 5,

      O::BitAnd => 6,
      O::BitOr => 6,
      O::BitXor => 6,

      O::And => 7,
      O::Or => 7,
      O::GreaterThan => 7,
      O::GreaterThanOrEq => 7,
      O::LessThan => 7,
      O::LessThanOrEq => 7,

      O::Equal => 8,
      O::NotEqual => 8,
    }
  }
}

impl Precedence for Op {
  fn precedence(&self) -> u8 {
    2
  }
}

#[allow(unused)]
pub enum TriOp {
  /// `a then b else c`
  ThenElse,

  /// `a |> b: c`
  Pipe
}

pub enum Expr {
  Value(IntermediateValue),
  // TriOperation(TriOp, Box<Expr>, Box<Expr>, Box<Expr>),
  BinOperation(BinOp, Box<Expr>, Box<Expr>),
  Operation(Op, Box<Expr>)
}

impl std::fmt::Debug for Expr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Expr::Value(val) => write!(f, "{:?}", val),
      Expr::BinOperation(op, left, right) => {
        f.debug_map()
          .entry(&"op", op)
          .entry(&"left", left)
          .entry(&"right", right)
          .finish()
      },
      Expr::Operation(op, expr) => {
        f.debug_map()
          .entry(&"op", op)
          .entry(&"expr", expr)
          .finish()
      },
    }
  }
}

// b if a else c
// a then b else c

/*
a then { b } else { c }
*/
