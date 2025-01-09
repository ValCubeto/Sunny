use std::fmt;
use crate::eval::tokenize::tokens::{ Token as Tk, Tokens };
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
  GetItem(E, E),
  /// `a . b`
  GetField(E, E),

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

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Expr::Single(value) => write!(f, "{value}"),
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
      Expr::GetField(lhs, rhs) => write!(f, "({lhs}.{rhs})"),
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
    Some(Tk::Ident(ident)) => Expr::Single(Value::Ident(ident.clone())),
    Some(Tk::String(string)) => Expr::Single(Value::String(string.clone())),
    Some(Tk::FString(fstring)) => Expr::Single(Value::FString(fstring.clone())),
    Some(Tk::Char(ch)) => Expr::Single(Value::Char(*ch)),
    Some(Tk::Number(number)) => Expr::Single(Value::Number(number.clone())),
    Some(Tk::LeftParen) => {
      let lhs = parse_expr_bp(tokens, 0);
      match tokens.next() {
        Some(Tk::RightParen) => {}
        Some(other) => syntax_err!("expected parenthesis, found {other}"),
        None => syntax_err!("unclosed parenthesis")
      }
      lhs
    }
    Some(Tk::Op(op)) => {
      let right_bp = op.prefix_bp();
      let rhs = parse_expr_bp(tokens, right_bp);
      op.prefix_expr(rhs)
    }
    Some(Tk::NewLine) => return parse_expr_bp(tokens, min_bp),
    Some(Tk::EoF) => syntax_err!("expected value"),
    Some(token) => syntax_err!("unexpected {token}")
  };
  loop {
    let op = match &tokens.peek_amount(2)[0..2] {
      [None, None] => break,
      [Some(Tk::NewLine), Some(Tk::Op(op))] => {
        tokens.next();
        op
      },
      [Some(Tk::NewLine), Some(Tk::NewLine)] => {
        tokens.next();
        tokens.next();
        while let Some(Tk::NewLine) = tokens.peek() {
          tokens.next();
        }
        continue;
      }
      [Some(Tk::Op(op)), _] => op,
      // Usually [Some(NewLine), _]
      _ => return lhs
    };
    if let Some(left_bp) = op.postfix_bp() {
      if left_bp < min_bp {
        break;
      }
      tokens.next();
      lhs = op.postfix_expr(lhs);
      continue;
    }
    if let Some((left_bp, right_bp)) = op.infix_bp() {
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
