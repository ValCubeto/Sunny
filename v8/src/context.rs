use std::str::Chars;
use crate::aliases::Id;

pub struct Context<'a> {
	id: Id,
	chars: Chars<'a>,
	current: char
}

impl<'a> Context<'a> {
	pub fn new(id: Id, data: &'a str) -> Self {
		let mut chars: Chars<'a> = data.chars();
		Context {
			id,
			current: chars.next().unwrap(),
			chars
		}
	}
}