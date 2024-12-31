use std::fmt::Display;

use crate::eval::tokenize::tokens::{ Token, Tokens };
use super::constants::Variable;
use super::values::Value;

type E = Box<Expr>;

#[allow(unused)]
#[derive(Debug)]
pub enum Expr {
  Single(Value),

  /// `!a`
  Not(E),
  /// `-a`
  Neg(E),
  /// `+a`
  Pos(E),
  /// `*a`
  Deref(E),
  /// `&a`
  Ref(E),
  /// `a%`
  Percent(E),
  /// `a?`
  Try(E),

  /// `a + b`
  Add(E, E),
  /// `a - b`
  Sub(E, E),
  /// `a * b`
  Mul(E, E),
  /// `a / b`
  Div(E, E),
  /// `a % b`
  Mod(E, E),
  /// `a && b`
  LogicalAnd(E, E),
  /// `a || b`
  LogicalOr(E, E),
  /// `a & b`
  And(E, E),
  /// `a | b`
  Or(E, E),
  /// `a ^ b`
  Xor(E, E),
  /// `a == b`
  Equal(E, E),
  /// `a != b`
  NotEqual(E, E),
  /// `a < b`
  Less(E, E),
  /// `a > b`
  Greater(E, E),
  /// `a <= b`
  LessEqual(E, E),
  /// `a >= b`
  GreaterEqual(E, E),
  /// `a << b`
  LeftShift(E, E),
  /// `a >> b`
  RightShift(E, E),
  /// `a <> b`
  Cmp(E, E),

  /// `a .. b`
  ExclusiveRange(E, E),
  /// `a ... b`
  InclusiveRange(E, E),

  /// `a = b`
  Assign(E, E),
  /// `a += b`
  AddAssign(E, E),
  /// `a -= b`
  SubAssign(E, E),
  /// `a *= b`
  MulAssign(E, E),
  /// `a /= b`
  DivAssign(E, E),
  /// `a %= b`
  ModAssign(E, E),
  /// `a **= b`
  PowAssign(E, E),
  /// `a &&= b`
  LogicalAndAssign(E, E),
  /// `a &= b`
  AndAssign(E, E),
  /// `a ||= b`
  LogicalOrAssign(E, E),
  /// `a |= b`
  OrAssign(E, E),
  /// `a ^= b`
  XorAssign(E, E),

  /// `a :: b`
  GetItem(E, String),
  /// `a . b`
  GetProp(E, String),

  // Call(ident, generics, args),
  Let(Box<Variable>),
  Var(Box<Variable>),

  Loop(Vec<Expr>),
  While(E, Vec<Expr>),
  If(E, Vec<Expr>, Vec<Expr>),
  ForIn(E, E, Vec<Expr>),
}

impl Expr {
  pub fn ptr(self) -> Box<Expr> {
    Box::new(self)
  }
}

impl Display for Expr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Expr::Single(value) => write!(f, "{value:?}"),
      Expr::Not(expr) => write!(f, "(!{expr})"),
      Expr::Neg(expr) => write!(f, "(-{expr})"),
      Expr::Pos(expr) => write!(f, "(+{expr})"),
      Expr::Deref(expr) => write!(f, "(*{expr})"),
      Expr::Ref(expr) => write!(f, "(&{expr})"),
      Expr::Percent(expr) => write!(f, "({expr}%)"),
      Expr::Try(expr) => write!(f, "({expr}?)"),
      Expr::Add(lhs, rhs) => write!(f, "({lhs} + {rhs})"),
      Expr::Sub(lhs, rhs) => write!(f, "({lhs} - {rhs})"),
      Expr::Mul(lhs, rhs) => write!(f, "({lhs} * {rhs})"),
      Expr::Div(lhs, rhs) => write!(f, "({lhs} / {rhs})"),
      Expr::Mod(lhs, rhs) => write!(f, "({lhs} % {rhs})"),
      _ => unimplemented!()
    }
  }
}

/// None if there are no tokens
pub fn parse_expr(tokens: &mut Tokens) -> Expr {
  parse_expr_bp(tokens, 0)
}

/// Parse expressions using a binding power algorithm
fn parse_expr_bp(tokens: &mut Tokens, min_bp: u8) -> Expr {
  // left-hand side
  let mut lhs = match tokens.next() {
    None => syntax_err!("unexpected end of file"),
    Some(Token::String(string)) => Expr::Single(Value::String(string.clone())),
    Some(Token::LeftParen) => {
      let lhs = parse_expr_bp(tokens, 0);
      if let Some(Token::RightParen) = tokens.next() {} else {
        syntax_err!("unclosed parenthesis");
      }
      lhs
    }
    Some(op) if op.is_op() => {
      let right_bp = prefix_bp(op);
      let rhs = parse_expr_bp(tokens, right_bp);
      op.prefix_expr(rhs)
    }
    Some(token) => syntax_err!("unexpected {token}")
  };
  loop {
    let op = match tokens.peek() {
      None => break,
      Some(&op) if op.is_op() => op,
      Some(_token) => return lhs
    };
    if let Some(left_bp) = postfix_bp(op) {
      if left_bp < min_bp {
        break;
      }
      tokens.next();
      lhs = op.postfix_expr(lhs);
      continue;
    }
    if let Some((left_bp, right_bp)) = infix_bp(op) {
      if left_bp < min_bp {
        break;
      }
      tokens.next();
      let rhs = parse_expr_bp(tokens, right_bp);
      lhs = op.infix_expr(lhs, rhs);
      continue;
    }
    break;
  }
  lhs
}

// It's an inferno to manually write all binding powers

fn prefix_bp(op: &Token) -> u8 {
  match op {
    | Token::Plus
    | Token::Minus => 9,
    _ => syntax_err!("unexpected {op}")
  }
}

// They usually use dummies
/// `Option<(u8, ())>`
fn postfix_bp(op: &Token) -> Option<u8> {
  match op {
    | Token::Question => Some(11),
    _ => None
  }
}

fn infix_bp(op: &Token) -> Option<(u8, u8)> {
  match op {
    | Token::Equal => Some((2, 1)),

    | Token::Plus
    | Token::Minus => Some((5, 6)),

    | Token::Star
    | Token::Slash
    | Token::Percent => Some((7, 8)),

    | Token::Dot => Some((14, 13)),

    _ => None
  }
}
