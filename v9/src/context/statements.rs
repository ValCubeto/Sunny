use super::{
  Context,
  expressions::parse_expr
};

impl<'a> Context<'a> {
  pub fn parse_statement(&mut self) {
    parse_expr(self);
  }
}
