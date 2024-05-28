use super::{Intermediate, Op, Parser};

parser_method! { fn parse_expr() -> Expr }

fn parse_expr(parser: &mut Parser) -> Expr {
  parser.next_token();
  println!("[idx={}] Parsing expr starting with: {:?}", parser.idx(), parser.current());
  let val = parser.parse_value();
  println!("{:?}", val);
  todo!()
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

