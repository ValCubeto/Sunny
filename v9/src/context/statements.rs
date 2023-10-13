use super::{
  Context,
  expressions::parse_expr
};

pub fn parse_statement(ctx: &mut Context) {
  parse_expr(ctx);
}