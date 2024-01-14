pub struct Context {
  pub chars: Vec<char>,
  pub current: char,
  pub cursor: usize
}
impl Context {
  pub fn new(string: &str) -> Self {
    Context {
      chars: string.chars().collect(),
      current: '\0',
      cursor: 0
    }
  }
  pub fn debug(&self) {
    println!("[{}/{}] {:?}", self.cursor, self.chars.len(), self.current);
  }
  pub fn next_char(&mut self) {
    self.cursor += 1;
    self.debug();
    if self.cursor == self.chars.len() - 1 {
      syn_error!("unexpected end of input")
    }
    self.current = self.chars[self.cursor];
  }
  /// Skips whitespaces, including new lines.
  pub fn skip_spaces(&mut self) {
    while self.current.is_whitespace() {
      self.next_char();
    }
  }
  /// Skips whitespaces, excluding new lines.
  /// Remember in this language semicolons are optional.
  pub fn skip_spacing(&mut self) {
    while self.current != '\n' && self.current.is_whitespace() {
      self.next_char();
    }
  }
  pub fn expect_current(&mut self, ch: char) {
    if self.current != ch {
      syn_error!("expected {ch:?}, got {:?}", self.current);
    }
  }
  /// Useful for composed tokens like "=>"
  pub fn expect_char(&mut self, ch: char) {
    self.next_char();
    self.expect_current(ch);
  }
  pub fn expect_token(&mut self, ch: char) {
    self.skip_spaces();
    self.expect_current(ch);
  }
  pub fn expect_word(&mut self) -> String {
    // self.skip_spaces();

    // Standard size of a small word, prevents reallocating multiple times.
    // If the word is more than 8 characters long, the capacity will grow
    // to 16.
    let mut word = String::with_capacity(8);

    if !self.current.is_alphabetic() {
      syn_error!("expected a word, found {:?}", self.current);
    }
    word.push(self.current);
    self.next_char();
    while self.current.is_alphanumeric() {
      word.push(self.current);
      self.next_char();
    }
    word
  }
}
