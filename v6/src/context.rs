use std::fmt::Display;

use crate::errors::InternalError;
// use crate::errors::SyntaxError;

pub struct Context {
	pub id: String,
	pub chars_raw: String, // for debugging
	pub chars: std::slice::Iter<'static, char>,
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
	pub fn new(id: String, chars: String) -> Self {
		Context {
			id,
			ch: chars.chars().collect::<Vec<char>>()[0],
			char_count: chars.len(),
			chars: chars.chars().collect::<Vec<char>>().iter(),
			chars_raw: chars,
			idx: 0,
			line: 0, // reserved for namespace name
			column: 1
		}
	}
	pub fn ignore_spaces(&mut self) {
		while self.idx < self.char_count {
			match self.ch {
				' ' | '\t' | '\n' | '\r' => {}
				// also ignore comments
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
	}
	pub fn next_char(&mut self) {
		match self.ch {
			'\n' => {
				self.column = 0;
				self.line += 1;
			}
			'\t' => {
				self.column += 4;
			}
			_ => {
				self.column += 1;
			}
		}
		self.idx += 1;
		self.ch = match self.chars.next() {
			None => {
				InternalError!("index go out of scope; idx = {}, len = {}", self.idx, self.char_count);
			}
			Some(ch) => *ch
		};
		// println!("next_char: {:?}, :{}:{}", self.ch, self.line, self.column);
	}
}