use std::fmt::Display;

use crate::errors::InternalError;
// use crate::errors::SyntaxError;

pub struct Context {
	pub id: String,
	pub char_vec: Vec<char>,
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
	pub fn new(id: String, chars: Vec<char>) -> Self {
		Context {
			id,
			ch: chars[0],
			char_count: chars.len(),
			chars: chars.iter(),
			char_vec: chars,
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
				InternalError!("");
			}
			Some(ch) => *ch
		};
		// println!("next_char: {:?}, :{}:{}", self.ch, self.line, self.column);
	}
}