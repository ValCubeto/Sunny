use unicode_segmentation::Graphemes;

use crate::{errors::SyntaxError, id::Id};

pub struct Context<'a> {
	id: Id,
	chars: Graphemes<'a>,
	idx: usize,
	line: usize,
	column: usize
}

impl<'a> Context<'a> {
	pub fn new(id: Id, chars: Graphemes<'a>) -> Self {
		Context {
			id,
			chars,
			idx: 0,
			line: 0,
			column: 1
		}
	}
	pub fn next_char(&mut self) -> &'a str {
		match self.chars.next() {
			Some(ch) => {
				ch
			}
			None => SyntaxError!(self, "")
		}
	}
}