/* use crate::types::IntermediateValue;

use super::{ parse_value, Parser };

pub fn parse_expr(parser: &mut Parser) -> Expr {
  let left = parse_value(parser);
  println!("Parsed value: {left:?}");
  println!();
  parser.next_token();
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
      parser.skip_whitespaces();
      let expr = match expr {
        Expr::BinOp(op, left, right) => {
          if op.precedence() < Op::Try.precedence() {
            Expr::Op(Op::Try, Expr::BinOp(op, left, right).boxed())
          } else {
            Expr::BinOp(op, left, Expr::Op(Op::Try, right).boxed())
          }
        }
        | Expr::Op(_, _)
        | Expr::Value(_) => {
            Expr::Op(Op::Try, expr.boxed())
          }
        // _ => todo!()
      };
      return parse_to_right(parser, expr);
    }
    '+' => BinOp::Add,
    '-' => BinOp::Sub,
    '/' => BinOp::Div,
    '%' => BinOp::Mod,
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
    '=' => {
      parser.next_char();
      parser.expect('=');
      BinOp::Equal
    }
    '!' => {
      parser.next_char();
      parser.expect('=');
      BinOp::NotEqual
    }
    '>' => {
      match parser.peek() {
        '=' => {
          parser.next_token();
          BinOp::GreaterThanOrEq
        }
        '>' => {
          parser.next_token();
          BinOp::RightShift
        }
        _ => BinOp::GreaterThan
      }
    }
    '<' => {
      // if
      //   matches!(
      //     left,
      //     | Expr::Value(IntermediateValue::Identifier(_))
      //     | Expr::BinOp(BinOp::GetProp, _)
      //     | Expr::BinOp(BinOp::GetItem, _)
      //   )
      // {
      //   //
      // }
      match parser.peek() {
        '=' => {
          parser.next_token();
          BinOp::LessThanOrEq
        }
        '<' => {
          parser.next_token();
          BinOp::LeftShift
        }
        _ => BinOp::LessThan
      }
    }
    // let the outer function decide what to do
    _ => return expr
  };
  parser.next_char();
  parser.next_token();
  println!("curr = {:?}", parser.current());
  let right = match expr {
    Expr::Value(_) => {
      let right = Expr::Value(parse_value(parser));
      Expr::BinOp(right_op, expr.boxed(), right.boxed())
    }
    Expr::Op(_, _) => {
      let right = Expr::Value(parse_value(parser));
      Expr::BinOp(right_op, expr.boxed(), right.boxed())
    }
    Expr::BinOp(left_op, left, right) => {
      let third = Expr::Value(parse_value(parser));
      // Less `a.b + c` => {prec:1} + {prec:2}
      //     (a.b) * c -> ((a.b) + c)
      // Equal `a.b.c`   => {prec:1} + {prec:1}
      //     (a.b) . c -> ((a.b) . c)
      // Great `a + b.c` => {prec:2} + {prec:1}
      //     (a * (b + c)) . c -> (a + (b.c))

      // 1 == 2        (eq, 1, 2)
      // 1 == 2 + 3    (eq, 1, (add, 2, 3))
      // 2 + 3         (add, 2, 3)
      // 2 + 3 == 1    (eq, 1, (add, 2, 3))
      // 1 + 2 == 3 + 4
      // (add, 1, 2)
      // (eq, (add, 1, 2), 3)
      // (eq, (add, 1, 2), (add, 3, 4))

      if left_op.precedence() > right_op.precedence() {
        let right = parse_to_right(parser, Expr::BinOp(right_op, right, third.boxed())).boxed();
        Expr::BinOp(
          left_op,
          left,
          right,
        )
      } else {
        let left = parse_to_right(parser, Expr::BinOp(left_op, left, right)).boxed();
        Expr::BinOp(
          right_op,
          left,
          third.boxed(),
        )
      }
    }
  };
  parser.next_token();
}

/// Ops that take two values
#[derive(Debug)]
pub enum BinOp {
  /// `a.b`
  GetProp,
  /// `a::b`
  GetItem,

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
  /// `a << b`
  LeftShift,
  /// `a >> b`
  RightShift,
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
      O::GetProp => 0,

      // Greater precedence than `Op`
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
      O::LeftShift => 5,
      O::RightShift => 5,

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
  // TriOp(TriOp, Box<Expr>, Box<Expr>, Box<Expr>),
  BinOp(BinOp, Box<Expr>, Box<Expr>),
  Op(Op, Box<Expr>)
}

impl Expr {
  pub fn op(&self) -> &BinOp {
    match self {
      Expr::BinOp(op, _, _) => op,
      _ => panic!()
    }
  }
  pub fn boxed(self) -> Box<Self> {
    Box::new(self)
  }
}

impl std::fmt::Debug for Expr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Expr::Value(val) => write!(f, "{:?}", val),
      Expr::BinOp(op, left, right) => {
        f.debug_map()
          .entry(&"op", op)
          .entry(&"left", left)
          .entry(&"right", right)
          .finish()
      },
      Expr::Op(op, expr) => {
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
 */
