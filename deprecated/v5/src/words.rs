use crate::errors::debug;

pub fn parse_word(chars: &[char], i: &mut usize) -> String {
	let mut word: String = String::from(chars[*i]);
	*i += 1;
	while *i < chars.len() {
		let chr: char = chars[*i];
		match chr {
			'a'..='z' | '_' | '0'..='9' | 'A'..='Z' => {
				word.push(chr);
			}
			_ => {
				break;
			}
		}
		*i += 1;
	}
	debug!("word = {word:?}");
	word
}