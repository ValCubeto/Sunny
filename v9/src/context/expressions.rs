use crate::{
  context::Context,
  debug_expr
};

/// # What is an expression?
/// - An identifier
/// - A literal value
/// - A struct initializer
/// - A function call
/// - An operation

impl<'a> Context<'a> {
  pub fn parse_expr(ctx: &mut Context) {
    loop {
      match ctx.current {
        n if n.is_ascii_digit() => {
          todo!();
          // let number = ctx.parse_number();
        }
        c if c.is_alphanumeric() => {
          let mut word = ctx.parse_word();
        }
        _ => break
      }
    }
  }
}