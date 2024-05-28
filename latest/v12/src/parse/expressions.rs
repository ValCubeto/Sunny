use super::{Op, Parser};

parser_method! { fn parse_expr() -> Expr }

fn parse_expr(parser: &mut Parser) -> Expr {
  todo!()
}

pub enum Expr {
  Single(Intermediate),
  Arith(Op, Box<Expr>, Box<Expr>),

}
impl Expr {
  #[inline(always)]
  pub fn ptr(self) -> Box<Expr> {
    Box::new(self)
  }
}

