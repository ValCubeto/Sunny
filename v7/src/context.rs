use std::str::Chars;
use crate::{errors::SyntaxError, id::Id};

pub struct Context<'a> {
	pub id: Id,
	pub code: &'a String,
	pub chars: Chars<'a>,
	pub current: char,
	pub idx: usize,
	pub line: usize,
	pub column: usize
}

impl<'a> Context<'a> {
	pub fn new(id: Id, code: &'a String) -> Self {
		let mut chars: Chars<'a> = code.chars();
		Context {
			id,
			code,
			current: chars.next().unwrap(),
			chars,
			idx: 0,
			line: 0,
			column: 1
		}
	}
	pub fn debug(&self) {
		println!("[{:?}:{}:{}] chars[{}] = {:?}", self.id, self.line, self.column, self.idx, self.current);
	}
	pub fn next_char(&mut self) {
		match self.chars.next() {
			Some(ch) => {
				match self.current {
					'\n' => {
						self.line += 1;
						self.column = 1;
					}
					'\t' => {
						self.column += 4;
					}
					_ => {
						self.column += 1;
					}
				}
				self.idx += 1;
				self.current = ch;
			}
			None => SyntaxError!(self, "unexpected end of input")
		}
	}
	pub fn go(&mut self) {
		loop {
			match self.current {
				' ' | '\n' | '\t' | '\r' => { /* ignore */ }
				'#' => {
					while self.current != '\n' {
						self.next_char();
					}
				}
				_ => break
			}
			self.next_char();
		}
	}
	pub fn collect_word(&mut self) -> Id {
		let mut word = String::from(self.current);
		self.next_char();
		loop {
			if !self.current.is_alphanumeric() {
				break;
			}
			word.push(self.current);
			self.next_char();
		}
		Id::from(word.as_str())
	}
	pub fn expect_word(&mut self) -> Id {
		if !self.current.is_alphanumeric() || self.current.is_ascii_digit() {
			SyntaxError!(self, "expected a word, got {:?}", self.current);
		}
		self.collect_word()
	}
}