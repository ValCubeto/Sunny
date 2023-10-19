use crate::context::Context;

#[allow(unused)]
impl<'a> Context<'a> {
  pub fn parse_number(&mut self) -> String {
    let mut number = String::from(self.current);
    number
  }
}

#[allow(unused)]
pub enum Number<'a> {
  Int(&'a str),
  Float(&'a str)
}