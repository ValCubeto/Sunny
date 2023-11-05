use crate::context::Context;

#[allow(unused)]
impl<'a> Context<'a> {
  pub fn parse_number(&mut self) -> Number {
    let mut number = String::new();
    if self.current == '0' {
      let next = self.peek();
      if next == 'x' {
        let mut hex = String::new();
        while self.current.is_ascii_hexdigit() {
          hex.push(self.current);
          self.next_char();
        }
        return Number::Hex(hex);
      }
      if next == 'b' {
        let mut bin = String::new();
        while self.current == '0' || self.current == '1' {
          bin.push(self.current);
          self.next_char();
        }
        return Number::Bin(bin);
      }
      self.next_char();
    }
    while self.current == '0' {    // skip zeros
      self.next_char();
    }
    while self.current.is_ascii_digit() {
      number.push(self.current);
      self.next_char();
      if matches!(self.current, '\'' | '_') {
        self.next_char();
      }
    }
    // peek() because of 123.to_string()
    // if self.current == '.' {
    //   self.next_char();
    //   // a
    //   return Number::Float(number)
    // }
    // if self.current.eq_ignore_ascii_case(&'e') {}
    Number::Int(number)
  }
}

#[allow(unused)]
#[repr(u8)]
pub enum Number {
  Bin(String),
  Hex(String),
  Float(String),
  Int(String),
  // Exponent(String, String) // 2e5 == 200000
}