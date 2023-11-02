use crate::context::Context;

#[allow(unused)]
impl<'a> Context<'a> {
  pub fn parse_number(&mut self) -> String {
    let mut number = String::from(self.current);
    if self.current == '0' {
      let next = self.peek();
      if next == 'x' {}
      if next == 'b' {}
      self.next_char();
    }
    // while self.current == '0' {    // skip zeros
    //   self.next_char();
    // }
    while self.current.is_ascii_digit() {
      number.push(self.current);
    }
    // if self.current == '.' {}
    // if self.current.eq_ignore_ascii_case(&'e') {}
    number
  }
}

#[allow(unused)]
pub enum Number<'a> {
  Int(&'a str),
  Float(&'a str)
}