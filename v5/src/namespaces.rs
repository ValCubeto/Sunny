use std::collections::HashMap;
use crate::types::Any;
use crate::errors::unexpected;
use crate::words::parse_word;

#[allow(unused)]
pub fn parse_namespace(chars: &[char], i: &mut usize) -> HashMap<String, Any<'static>> {
	let namespace: HashMap<String, Any> = HashMap::new();
	while *i < chars.len() {
		let chr: char = chars[*i];
		match chr {
			'\n' | ' ' | '\t' | '\r' => {
				// ignore
			}
			'a'..='z' | '_' | 'A'..='Z' => {
				let word: String = parse_word(chars, i);
			}
			_ => {
				unexpected(chr);
			}
		}
		*i += 1;
	}
	namespace
}