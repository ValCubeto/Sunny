use std::{ collections::HashSet, iter::Peekable, str::Chars };
use once_cell::sync::Lazy;

static KEYWORDS: Lazy<HashSet<&str>> = Lazy::new(|| HashSet::from([
  "mod",
  "pub",
  "priv",
  "self",
  "Self",
  "const",
  "fun",
  "class",
  "enum",
  "struct",
  "trait",
  "impl",
  "typedef",
  "flagset",

  "let",
  "var",

  "if",
  "then",
  "else",

  "loop",
  "while",
  "for",

  "return",
  "break",
  "continue",

  "use",
  "as",
  "from",

  "new",
  "match",

  "with",
  "unsafe",
  "async",
  "await",
  "defer",
]));

/// I think the name is enough descriptive
pub struct Parser<'a> {
  pub file_name: &'a str,
  data_len: usize,
  data: Peekable<Chars<'a>>,
  pub idx: usize,
  /// # IMPORTANT
  /// If you modify it, make sure to call `self.update_file_pos()` after.
  current: char,
  pub line: usize,
  pub column: usize
}

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
      column: 1,
    };
    this.update_file_pos();
    this
  }

  /// Panics if the token is a keyword
  #[inline(always)]
  pub fn check_keyword(&self, word: &str) {
    if KEYWORDS.contains(word) {
      syntax_err!("unexpected keyword {word:?} here"; self);
    }
  }

  /// Returns `self.current`. This prevents modifications from outside.
  #[inline(always)]
  pub fn current(&self) -> char {
    self.current
  }

  /// This function MUST be called after of directly updating
  /// `self.current`
  /// Updates the file position (`self.line` and `self.column`).
  fn update_file_pos(&mut self) {
    match self.current {
      '\n' => {
        self.column = 1;
        self.line += 1;
      }
      '\t' => {
        // HINT: let the user decide how many spaces a tab uses.
        self.column += 4;
      }
      _ => {
        self.column += 1;
      }
    }
  }

  /// Returns a copy of the next character without
  /// advancing the cursor
  pub fn peek(&mut self) -> char {
    *(self.data.peek()
      .unwrap_or_else(|| syntax_err!("unexpected end of input")))
  }

  /// Used to prevent recursion by `self.next_char`
  #[inline]
  fn _next_char(&mut self) {
    self.idx += 1;
    if self.idx >= self.data_len {
      syntax_err!("unexpected end of input"; self);
    }
    self.current = self.data.next().unwrap();
    self.update_file_pos();
  }

  /// Goes to the next character and returns it.
  /// Panics if the input ends.
  pub fn next_char(&mut self) {
    self.idx += 1;
    if self.idx >= self.data_len {
      syntax_err!("unexpected end of input"; self);
    }
    self.current = self.data.next().unwrap();
    self.update_file_pos();

    // Peeks so the current keeps being '/'
    if self.current == '/' {
      let peeked = self.peek();
      if peeked == '/' {
        self._next_char();
        self._next_char();
        while self.current != '\n' {
          self._next_char();
        }
      } else if peeked == '*' {
        self._next_char();
        self._next_char();
        loop {
          if self.current == '*' && self.peek() == '/' {
            self._next_char();
            self._next_char();
            break;
          }
          self._next_char();
        }
      }
    }
    println!("    {}: {:?}", self.idx, self.current);
  }

  /// Skips spaces, excluding the end of line. Panics if the input ends.
  pub fn skip_spaces(&mut self) {
    while matches!(self.current, ' ' | '\t' | '\r') {
      self.next_char();
    }
  }

  /// Goes to the next character until there is a non-whitespace character,
  /// or finishes the program when reaching the end of the input.
  pub fn skip_whitespaces(&mut self) {
    // NOTE: other types of whitespace will be considered invalid.
    while matches!(self.current, ' ' | '\n' | '\t' | '\r') {
      self.idx += 1;
      if self.idx >= self.data_len {
        std::process::exit(0);
      }
      self.current = self.data.next().unwrap();
      self.update_file_pos();
    }
  }

  /// Similar to `self.skip_whitespaces`, but matches new lines.
  pub fn next_token(&mut self) {
    while matches!(self.current, ' ' | '\t' | '\n' | '\r') {
      self.next_char();
    }
  }

  /// Used when an alphabetic character is found.
  /// Returns it + the next alphanumeric characters, if any.
  #[must_use]
  pub fn parse_word(&mut self) -> String {
    let mut word = String::from(self.current);
    self.next_char();
    // TODO: I should look for ascii characters, then the underscore,
    // and finally other alphanumeric characters.
    while self.current.is_alphanumeric() || self.current == '_' {
      word.push(self.current);
      self.next_char();
    }
    word
  }

  /// Used when a keyword is expected. Similar to `Parser::parse_word`.
  #[must_use]
  pub fn parse_ascii_word(&mut self) -> String {
    let mut word = String::from(self.current);
    self.next_char();
    while self.current.is_ascii_alphabetic() {
      word.push(self.current);
      self.next_char();
    }
    if self.current.is_alphanumeric() {
      syntax_err!("unexpected identifier `{word}{}` here", self.parse_word(); self);
    }
    word
  }

  /// Expects the current character to be a valid identifier character,
  /// and then calls `Self::parse_word`.
  #[must_use]
  pub fn expect_word(&mut self) -> String {
    // NOTE: `is_alphanumeric` includes ascii digits.
    // Doesn't allow the first character to be an ascii digit.
    // For example, `2dVector` will be invalid.
    if self.current.is_ascii_digit() || !self.current.is_alphanumeric() {
      syntax_err!("expected an identifier, found {:?}", self.current; self)
    }
    self.parse_word()
  }

  /// Errors if the current character is not the expected.
  /// If it is, goes to the next character.
  pub fn expect(&mut self, expected: char) {
    if self.current != expected {
      syntax_err!("expected {expected:?}, but got {:?}", self.current; self);
    }
    self.next_char();
  } 
}
