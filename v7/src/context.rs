use std::str::Chars;
use crate::{errors::SyntaxError, id::Id};

pub struct Context<'a> {
	pub id: Id,
	pub data: &'a String,
	pub chars: Chars<'a>,
	pub current: char,
	pub idx: usize,
	pub line: usize,
	pub column: usize
}

impl<'a> Context<'a> {
	pub fn new(id: Id, data: &'a String) -> Self {
		Context {
			id,
			data,
			current: data.chars().next().unwrap(),
			chars: data.chars(),
			idx: 0,
			line: 0,
			column: 1
		}
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
		}
	}
	pub fn collect_word(&self) -> Id {
		let mut word = String::from(self.current);
		loop {
			if !self.current.is_alphanumeric() {
				break;
			}
			word.push(self.current);

		}
		Id::from(word.as_str())
	}
}