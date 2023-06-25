use std::fmt::Display;
// use crate::errors::SyntaxError;

pub struct Context {
	pub id: String,
	pub chars: Vec<char>,
	pub ch: char,
	pub char_count: usize,
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
			char_count: chars.len(),
			chars,
			idx: 0,
			line: 1,
			column: 1
		}
	}
	pub fn ignore_spaces(&mut self) {
		// let start = (self.line, self.column);
		while self.idx < self.char_count {
			match self.ch {
				' ' | '\t' | '\n' | '\r' => {}
				'#' => {
					while self.idx < self.char_count && self.ch != '\n' {
						self.next_char();
					}
				}
				_ => {
					break;
				}
			}
			self.next_char();
		}
		// println!("ignored spaces from {}:{} to {}:{}, break at {:?}", start.0, start.1, self.line, self.column, self.ch);
	}
	pub fn next_char(&mut self) {
		match self.ch {
			'\n' => {
				self.line += 1;
				self.column = 1;
			}
			| 'a'..='z'
			| 'A'..='Z'
			| '_'
			| ' ' | '\t' | '\r' => {}
			_ => {
				// SyntaxError!(self, "unexpected character {:?}", self.ch);
			}
		}
		if self.ch == '\n' {
		} else {
			self.column += 1;
		}
		self.idx += 1;
		self.ch = self.chars[self.idx];
		// println!("next_char: {:?}, :{}:{}", self.ch, self.line, self.column);
	}
}