use super::Parser;
use crate::lang::IntermediateValue;


pub fn parse_expr(parser: &mut Parser) -> Expr {
  //
}

// Ends if EOL is found
pub fn parse_expr_to_end(parser: &mut Parser) -> Expr {
  //
}

pub enum Expr {
  Value(IntermediateValue),
  // TriOp(TriOp, Box<Expr>, Box<Expr>, Box<Expr>),
  Operation(Operator, Box<Expr>, Box<Expr>),
}

pub enum Operator {
  /// `x::y`
  GetItem,
  /// `x.y`
  GetProp,

  /// `&x`
  Ref,
  /// `*x`
  Deref,
  
  /// `x?`
  Try,

  /// `-x`
  Negate,
  /// `!x`
  Not,

  /// `x ** y`
  Pow,

  /// `x % y`
  Mod,
  /// `x * y`
  Mul,
  /// `x / y`
  Div,
  
  /// `x + y`
  Add,
  /// `x - y`
  Sub,

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

  /// `x > y`
  GreaterThan,
  /// `x >= y`
  GreaterEq,
  /// `x < y`
  LessThan,
  /// `x <= y`
  LessEq,

  /// `x == y`
  Equal,
  /// `x != y`
  NotEqual,

  /// `x && y`
  And,
  /// `x || y`
  Or,
}
impl Operator {
  #[inline(always)]
  pub fn precedence(&self) -> u8 {
    // `a = Some(true)`
    // `!*&a?`
    match self {
      Self::GetItem | Self::GetProp => 0,
      Self::Ref | Self::Deref => 1,
      Self::Try => 2,
      Self::Negate | Self::Not => 3,
      Self::Pow => 4,
      Self::Mod | Self::Mul | Self::Div => 6,
      Self::Add | Self::Sub => 7,
      Self::BitAnd | Self::BitOr | Self::BitXor => 8,
      Self::ShiftLeft | Self::ShiftRight => 9,
      Self::GreaterThan | Self::GreaterEq => 10,
      Self::LessThan | Self::LessEq => 11,
      Self::Equal | Self::NotEqual => 12,
      Self::And | Self::Or => 13,
    }
  }
}
