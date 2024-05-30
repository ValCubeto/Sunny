use super::{parser, Parser};

parser_method! { fn parse_op() -> Option<Op> }

fn parse_op(parser: &mut Parser) -> Option<Op> {
  let prev = parser.current();
  parser.next_char();
  let curr = parser.current();
  let op = match prev {
    '+' => match curr {
      '=' => Op::AddAssign,
      _ => Op::Add
    },
    '-' => match curr {
      '=' => Op::SubAssign,
      _ => Op::Sub
    },
    '*' => match curr {
      '*' if parser.peek() == '=' => Op::PowAssign,
      '*' => Op::Pow,
      '=' => Op::MulAssign,
      _ => Op::Mul
    },
    '/' => match curr {
      '=' => Op::DivAssign,
      _ => Op::Div
    },
    '%' => match curr {
      '=' => Op::ModAssign,
      _ => Op::Mod
    },
    '<' => match curr {
      '=' => Op::LessOrEq,
      _ => Op::LessThan
    },
    '>' => match curr {
      '=' => Op::GreaterOrEq,
      _ => Op::GreaterThan
    },
    '=' => match curr {
      '=' if parser.peek() == '=' => Op::Is,
      '=' => Op::Eq,
      _ => Op::Assign
    },
    '!' => match curr {
      '=' if parser.peek() == '=' => Op::IsNot,
      '=' => Op::NotEq,
      _ => panic!("expected '!=' or '!=='")
    },
    '&' => match curr {
      '&' if parser.peek() == '=' => Op::AndAssign,
      '&' => Op::And,
      '=' => Op::BitAndAssign,
      _ => Op::BitAnd
    },
    '|' => match curr {
      '|' if parser.peek() == '=' => Op::OrAssign,
      '|' => Op::Or,
      '=' => Op::BitOrAssign,
      _ => Op::BitOr
    },
    '^' => match curr {
      '=' => Op::BitXorAssign,
      _ => Op::BitXor
    },
    '.' if curr != '.' => panic!("expected '..' or '...'"),
    '.' => match parser.peek() {
      '.' => Op::InclusiveRange,
      _ => Op::ExclusiveRange,
    },
    _ => return None
  };
  Some(op)
}

#[derive(Debug, Copy, Clone)]
pub enum Op {
  /// `x::y`
  GetItem,
  /// `x.y`
  GetProp,
  
  /// `&x`
  Ref,
  /// `*x`
  Deref,

  /// `!x`
  Not,
  // `-x`
  Neg,
  /// `x?`
  Try,

  /// `x..y`
  ExclusiveRange,
  /// `x...y`
  InclusiveRange,

  /// `x & y`
  BitAnd,
  /// `x | y`
  BitOr,
  /// `x ^ y`
  BitXor,
  /// `x << y`
  ShiftLeft,
  /// `x >> y`
  ShiftRight,

  /// `x ** y`
  Pow,

  /// `x * y`
  Mul,
  /// `x / y`
  Div,
  /// `x % y`
  Mod,

  /// `x + y`
  Add,
  /// `x - y`
  Sub,

  /// `x < y`
  LessThan,
  /// `x > y`
  GreaterThan,
  /// `x <= y`
  LessOrEq,
  /// `x >= y`
  GreaterOrEq,
  /// `x === y`
  Is,
  /// `x !== y`
  IsNot,
  /// `x == y`
  Eq,
  /// `x != y`
  NotEq,

  /// `x && y`
  And,
  /// `x || y`
  Or,

  /// `x = y`
  Assign,
  /// `x += y`
  AddAssign,
  /// `x -= y`
  SubAssign,
  /// `x *= y`
  MulAssign,
  /// `x /= y`
  DivAssign,
  /// `x %= y`
  ModAssign,
  /// `x **= y`
  PowAssign,
  /// `x &&= y`
  AndAssign,
  /// `x ||= y`
  OrAssign,
  /// `x &= y`
  BitAndAssign,
  /// `x |= y`
  BitOrAssign,
  /// `x ^= y`
  BitXorAssign,
  /// `x <<= y`
  ShiftLeftAssign,
  /// `x >>= y`
  ShiftRightAssign,
}

impl Op {
  /// Returns the precedence of the operator.
  /// The lower the precedence, the higher the priority.
  #[inline(always)]
  pub fn prec(&self) -> u8 {
    match self {
      Op::GetItem => 0,
      Op::GetProp => 0,
      _ => todo!()
    }
  }
}
