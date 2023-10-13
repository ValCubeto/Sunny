use crate::context::Context;

impl<'a> Context<'a> {
  pub fn parse_statement(&mut self) {
    self.parse_expr();
  }
}
