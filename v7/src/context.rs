use std::{str::Chars, collections::HashMap, rc::Rc};
use crate::{ SyntaxError,
	id::Id,
	functions::Function,
	arguments::Arguments,
	eval::eval_ast,
	values::Value };

pub struct Context<'a> {
	pub id: Id,
	pub code: &'a String,
	pub chars: Chars<'a>,
	pub current: char,
	pub idx: usize,
	pub line: usize,
	pub column: usize,
	pub stack: Vec<HashMap<Id, Value>>
}

impl<'a> Context<'a> {
	pub fn new(id: Id, code: &'a String) -> Self {
		let mut chars: Chars<'a> = code.chars();
		Context {
			stack: Vec::new(),
			id,
			code,
			current: chars.next().unwrap(),
			chars,
			idx: 0,
			line: 0,
			column: 1,
		}
	}
	#[allow(unused)]
	pub fn debug(&self) {
		println!("[{:?}:{}:{}] chars[{}] = {:?}", self.id, self.line, self.column, self.idx, self.current);
	}
	#[allow(clippy::boxed_local)]
	pub fn call_fun(&mut self, function: Box<Function>, args: Arguments) -> Value {
		self.stack.push(HashMap::new());
		eval_ast(&(function.body), args, self)
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
	#[allow(unused)]
	pub fn skip_spaces(&mut self) {
		loop {
			if matches!(self.current, ' ' | '\t' | '\r') {
				self.next_char();
				continue;
			}
			if self.current == '#' {
				while self.current != '\n' {
					self.next_char();
				}
				continue;
			}
			break;
		}
	}
	pub fn go(&mut self) {
		loop {
			if matches!(self.current, ' ' | '\t' | '\r' | '\n') {
				self.next_char();
				continue;
			}
			if self.current == '#' {
				while self.current != '\n' {
					self.next_char();
				}
				continue;
			}
			break;
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
	pub fn is_valid_id(&self) -> bool {
		self.current.is_alphanumeric() && !self.current.is_ascii_digit()
	}
	pub fn expect_word(&mut self) -> Id {
		if !self.is_valid_id() {
			SyntaxError!(self, "expected a word, found {:?}", self.current);
		}
		self.collect_word()
	}
}