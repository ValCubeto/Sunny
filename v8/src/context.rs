use std::str::Chars;
use crate::aliases::Id;

pub struct Context<'a> {
	pub id: Id,
	pub chars: Chars<'a>,
	pub current: char,
	pub line: usize,
	pub column: usize,
}

impl<'a> Context<'a> {
	pub fn new(id: Id, data: &'a str) -> Self {
		let mut chars: Chars<'a> = data.chars();
		Context {
			id,
			current: chars.next().unwrap(),
			chars,
			line: 0,
			column: 1
		}
	}
}