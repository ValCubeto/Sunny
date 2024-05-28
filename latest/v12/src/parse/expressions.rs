use super::{Intermediate, Op, Parser};

parser_method! { fn parse_expr() -> Expr }

fn parse_expr(parser: &mut Parser) -> Expr {
  parser.next_token();
  let val: Expr = parser.parse_value();
  parser.next_token();
  // ensure there is an operator, else return here
  let Some(op1) = parser.parse_op() else {
    return val;
  };
  parser.next_token();
  let right = parser.parse_value();

  let Some(op2) = parser.parse_op() else {
    return Expr::Arith(op, val.ptr(), right.ptr())
  };
  if op1.prec() < op2.prec() {
   //
  }
}

#[derive(Debug)]
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
}

