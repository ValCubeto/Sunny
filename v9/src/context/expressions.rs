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

#[allow(unused)]
pub fn parse_expr(ctx: &mut Context) {
  loop {
    match ctx.current {
      n if n.is_ascii_digit() => {}
      c if c.is_alphabetic() => {
        let mut word = 
        debug_expr!(c);
        ctx.next_char();
      }
      _ => break
    }
  }
}