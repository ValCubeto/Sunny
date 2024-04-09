use std::{ iter::Peekable, str::Chars };

/// I think the name is enough descriptive
pub struct Parser<'a> {
  pub file_name: &'a str,
  data_len: usize,
  data: Peekable<Chars<'a>>,
  pub idx: usize,
  pub current: char,

  pub line: usize,
  pub column: usize,
}

// FIXME: where the 'ignore comments' section should be?
impl<'a> Parser<'a> {
  pub fn new(file_name: &'a str, data: &'a str) -> Self {
    let mut chars = data.chars().peekable();
    let mut this = Parser {
      file_name,
      current: chars.next().unwrap(),
      data_len: data.len(),
      data: chars,
      idx: 1,
      line: 1,
      column: 1
    };
    this.update_file_pos();
    this
  }

  /// This function MUST be called instead of directly updating `self.current`
  /// Updates the file pos (`self.line` and `self.column`)
  pub fn update_file_pos(&mut self) {
    match self.current {
      '\n' => {
        self.column = 1;
        self.line += 1;
      }
      '\t' => {
        // HINT: let the user decide how many spaces a tab uses
        self.column += 4;
      }
      _ch => {
        self.column += 1;
      }
    }
  }

  pub fn peek(&mut self) -> char {
    *(self.data.peek().expect("unexpected end of input"))
  }

  /// Goes to the next character and returns it. Panics if the input ends
  pub fn next_char(&mut self) {
    self.idx += 1;
    if self.idx >= self.data_len {
      syntax_err!("unexpected end of input"; self);
    }
    self.current = self.data.next()
      .expect("unexpected end of input");
    self.update_file_pos();
  }

  /// Skips spaces, excluding the end of line. Panics if the input ends
  pub fn skip_spaces(&mut self) {
    while matches!(self.current, ' ' | '\t') {
      self.next_char();
    }
  }

  /// Goes to the next character until there is a non-whitespace character,
  /// or finishes the program when reaching the end of the input
  pub fn skip_whitespaces(&mut self) {
    // NOTE: other types of whitespace will be considered invalid
    while matches!(self.current, ' ' | '\n' | '\t' | '\r') {
      self.idx += 1;
      if self.idx >= self.data_len {
        std::process::exit(0);
      }
      self.current = self.data.next().unwrap();
      self.update_file_pos();
    }
  }

  /// Similar to `self.skip_whitespaces`, but matches new lines
  pub fn next_token(&mut self) {
    while matches!(self.current, ' ' | '\t' | '\n' | '\r') {
      self.next_char();
    }
  }

  /// Used when an alphabetic character is found. Returns it + the next alphanumeric characters, if any
  #[must_use]
  pub fn parse_word(&mut self) -> String {
    let mut word = String::from(self.current);
    self.next_char();
    // TODO: I should look for ascii characters, then the underscore,
    // and finally other alphanumeric characters
    while self.current.is_alphanumeric() || self.current == '_' {
      word.push(self.current);
      self.next_char();
    }
    word
  }

  /// Used when a keyword is expected. Similar to `Parser::parse_word`
  #[must_use]
  pub fn parse_ascii_word(&mut self) -> String {
    let mut word = String::from(self.current);
    self.next_char();
    while self.current.is_ascii_alphabetic() {
      word.push(self.current);
      self.next_char();
    }
    word
  }

  /// Expects the current character to be a valid identifier character, and then calls `Self::parse_word`
  #[must_use]
  pub fn expect_word(&mut self) -> String {
    // NOTE: `is_alphanumeric` includes ascii digits
    if self.current.is_ascii_digit() || !self.current.is_alphanumeric() {
      syntax_err!("expected an identifier, found {:?}", self.current; self)
    }
    self.parse_word()
  }

  pub fn expect(&mut self, expected: char) {
    if self.current != expected {
      syntax_err!("expected {expected:?}, but got {:?}", self.current; self);
    }
    self.next_char();
  } 
}
