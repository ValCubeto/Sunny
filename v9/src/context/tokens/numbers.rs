use crate::context::Context;

impl<'a> Context<'a> {
  pub fn parse_number(&mut self) -> String {
    let mut number = String::from(self.current);
    number
  }
}

pub enum Number<'a> {
  Int(&'a str),
  Float(&'a str)
}