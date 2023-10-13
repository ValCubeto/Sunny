use crate::context::Context;

/// # What is an expression?
/// - An identifier
/// - A literal value
/// - A struct initializer
/// - A function call
/// - An operation

impl<'a> Context<'a> {
  pub fn parse_expr(&mut self) {
    loop {
      match self.current {
        n if n.is_ascii_digit() => {
          todo!();
          // let number = self.parse_number();
        }
        c if c.is_alphanumeric() => {
          let mut word = self.parse_word();
          todo!()
        }
        _ => break
      }
    }
  }
}