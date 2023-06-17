use std::process::exit;

use crate::colors::error;

pub struct Context {
	pub id: String,
	pub chars: Vec<char>,
	pub ch: char,
	pub idx: usize,
	pub line: usize,
	pub column: usize
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
		self.idx += 1;
		self.ch = self.chars[self.idx];
		if self.ch == '\n' {
			self.line += 1;
			self.column = 1;
		}
	}
	pub fn throw<'a, S>(&self, ename: &'a str, message: S)
	where
		String: From<S>
	{
		println!("{}: {}", error(ename), String::from(message));
		println!("    at {}:{}:{}", self.id, self.line, self.column);
		exit(1);
	}
}