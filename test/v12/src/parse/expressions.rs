use super::{Intermediate, Op, Parser};

parser_method! { fn parse_expr(prev: Expr) -> Expr }

fn parse_expr(parser: &mut Parser, prev: Expr) -> Expr {
  // ensure there is an operator, else return here
  let op = match parser.parse_op() {
    Some(op) => op,
    None => return prev
  };
  if op.prec() < prev.op().prec() {
    Expr::Arith(
      *prev.op(),
      prev.left().clone().ptr(),
      parser.parse_expr(prev.right().clone()).ptr()
    )
  } else {
    let value = parser.parse_value();
    Expr::Arith(
      op,
      prev.clone().ptr(),
      parser.parse_expr(value).ptr()
    )
  }
}

#[derive(Debug, Clone)]
pub enum Expr {
  Single(Intermediate),
  Unary(Op, Box<Expr>),
  Arith(Op, Box<Expr>, Box<Expr>),
}
impl Expr {
  #[inline(always)]
  pub fn ptr(self) -> Box<Expr> {
    Box::new(self)
  }

  pub fn op(&self) -> &Op {
    match self {
      Expr::Single(_) => unreachable!(),
      Expr::Unary(op, _) => op,
      Expr::Arith(op, _, _) => op,
    }
  }
  pub fn left(&self) -> &Expr {
    match self {
      Expr::Single(_) => unreachable!(),
      Expr::Unary(_, left) => left,
      Expr::Arith(_, left, _) => left,
    }
  }
  pub fn right(&self) -> &Expr {
    match self {
      Expr::Single(_) => unreachable!(),
      Expr::Unary(_, _) => unreachable!(),
      Expr::Arith(_, _, right) => right,
    }
  }
}

