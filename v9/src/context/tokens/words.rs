use crate::{context::Context, syntax_error};

#[allow(unused)]
impl<'a> Context<'a> {
  pub fn parse_word(&mut self) -> String {
    let mut word = String::from(self.current);
    while self.current.is_alphanumeric() {
      word.push(self.current);
      self.next_char();
    }
    word
  }

  pub fn expect_word(&mut self) -> String {
    if !self.current.is_alphabetic() {
      syntax_error!("expected an identifier, got {:?}", self.current; self);
    }
    self.parse_word()
  }
}