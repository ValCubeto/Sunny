use super::Context;

/// # What is an expression?
/// - An identifier
/// - A literal value
/// - A struct initializer
/// - A function call
/// - An operation between expressions
/// - A path

#[allow(unused)]
impl<'a> Context<'a> {
  pub fn parse_expr(&mut self) {
    loop {
      match self.current {
        n if n.is_ascii_digit() => {
          let number = self.parse_number();
          todo!();
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