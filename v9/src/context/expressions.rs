use crate::{
  context::Context,
  debug_expr
};

#[allow(unused)]
pub fn parse_expr(ctx: &mut Context) {
  loop {
    match ctx.current {
      id if id.is_alphabetic() => {
        debug_expr!(id);
        ctx.next_char();
      }
      _ => break
    }
  }
}