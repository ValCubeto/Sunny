use std::str::Chars;

pub struct Parser<'a> {
  pub file_name: &'a str,
  data_len: usize,
  data: Chars<'a>,
  pub idx: usize,
  pub current: char,

  pub line: usize,
  pub column: usize,
}

/// Where the 'ignore comments' section should be?
impl<'a> Parser<'a> {
  pub fn new(file_name: &'a str, data: &'a str) -> Self {
    let mut chars = data.chars();
    Parser {
      file_name,
      current: chars.next().unwrap(),
      data_len: data.len(),
      data: chars,
      idx: 1,
      line: 0,
      column: 0
    }
  }

  /// Goes to the next character and returns it. Panics if the input ends
  pub fn next_char(&mut self) {
    self.idx += 1;
    if self.idx >= self.data_len {
      syn_err!("unexpected end of input");
    }
    self.current = self.data.next().expect("unexpected end of input");
    match self.current {
      '\n' => {
        self.column = 1;
        self.line += 1;
      }
      '\t' => {
        self.column += 4;
      }
      _ch => {
        self.column += 1;
      }
    }
  }

  /// Skips spaces, excluding the end of line. Panics if the input ends
  pub fn skip_spaces(&mut self) {}

  /// Goes to the next character until there is a non-whitespace character, or exits at the end of the input
  pub fn skip_whitespaces(&mut self) {
    // Other types of whitespace will be considered invalid
    while matches!(self.current, ' ' | '\n' | '\t' | '\r') {
      self.idx += 1;
      if self.idx >= self.data_len {
        std::process::exit(0);
      }
      self.current = self.data.next().unwrap();
    }
  }

  /// Similar to `skip_whitespaces`, but panics if the input ends
  pub fn next_token(&mut self) {}

  /// Used when an alphabetic character is found. Returns it + the next alphanumeric characters, if any
  #[must_use]
  pub fn parse_word(&mut self) -> String {
    let mut word = String::from(self.current);
    self.next_char();
    while self.current.is_alphanumeric() {
      word.push(self.current);
      self.next_char();
    }
    word
  }
}
