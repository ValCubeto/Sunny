use std::fmt::Display;

pub struct Context {
	pub id: String,
	pub chars: Vec<char>,
	pub ch: char,
	pub idx: usize,
	pub line: usize,
	pub column: usize
}

impl Display for Context {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{}:{} - chars[{}] = {:?}", self.id, self.line, self.column, self.idx, self.ch)
	}
}

impl Context {
	pub fn new(id: String, chars: Vec<char>) -> Self {
		Context {
			id,
			ch: chars[0],
			chars,
			idx: 0,
			line: 1,
			column: 1
		}
	}
	pub fn next_char(&mut self) {
		if self.ch == '\n' {
			self.line += 1;
			self.column = 1;
		} else {
			self.column += 1;
		}
		self.idx += 1;
		self.ch = self.chars[self.idx];
		// println!("next_char: {:?}, :{}:{}", self.ch, self.line, self.column);
	}
}