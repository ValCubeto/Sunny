use crate::{
  context::Context,
  syntax_error
};

impl<'a> Context<'a> {
  pub fn parse_statement(&mut self) {
    // match self.current {
    //   c if c.is_ascii_digit() => todo!(),
    //   c if c.is_alphabetic() => {
    //     let word = self.parse_word();
    //     match word.as_str() {
    //       "var" => todo!(),
    //       _ => self.parse_expr_with_token(Token::Ident(&word))
    //     }
    //   }
    //   _ => syntax_error!("unexpected character {:?}", self.current; self)
    // }
  }
}
