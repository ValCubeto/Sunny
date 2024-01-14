pub struct Context {
  chars: Vec<char>
}
impl Context {
  pub fn new(string: &str) -> Self {
    Context {
      chars: string.chars().collect()
    }
  }
  pub fn next_char(&mut self) -> char {
    todo!()
  }
  pub fn skip_spaces(&mut self) {}
  pub fn expect_char(&mut self, ch: char) {
    todo!()
  }
  pub fn expect_word(&mut self) -> String {
    self.skip_spaces();
    let word = String::with_capacity(8);

    todo!();
  }
}
