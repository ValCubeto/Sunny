use std::str::Chars;
use crate::{errors::SyntaxError, id::Id};

pub struct Context<'a> {
	id: Id,
	data: String,
	chars: Chars<'a>,
	current: char,
	idx: usize,
	line: usize,
	column: usize
}

impl<'a> Context<'a> {
	pub fn new(id: Id, data: String) -> Self {
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
	pub fn next_char(&mut self, expect_end: bool) {
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
			None => if !expect_end {
				SyntaxError!(self, "unexpected end of input");
			}
		}
	}
	pub fn go(&mut self, expect_end: bool) {
		loop {
			match self.current {
				' ' | '\n' | '\t' | '\r' => { /* ignore */ }
				'#' => {
					while self.current != '\n' {
						self.next_char(expect_end);
					}
				}
				_ => break
			}
		}
	}
}