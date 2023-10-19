/* use crate::{context::Context, syntax_error};

use super::expressions::Token;

impl<'a> Context<'a> {
  pub fn parse_statement(&mut self) {
    match self.current {
      c if c.is_alphabetic() => {
        let word = self.parse_word();
        match word.as_str() {
          "if" => todo!(),
          _ => self.parse_expr_with_token(Token::Ident(&word))
        }
      }
      _ => syntax_error!("unexpected character {:?}", self.current; self)
    }
  }
}
 */