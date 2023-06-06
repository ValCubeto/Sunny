use crate::errors::SyntaxError;

pub struct Module {}

impl Module {
	// pub fn exec_function(name: String, arguments: Arguments) {}
}

pub fn parse_module(code: String, id: String) -> Module {
	let code: Vec<char> = code.chars().collect();
	let mut ch: char;
	let mut idx: usize = 0;

	#[allow(unused)]
	let mut line: usize = 1;
	#[allow(unused)]
	let mut column: usize = 1;

	while idx < code.len() {
		ch = code[idx];
		match ch {
			'\n' => {
				line += 1;
				column = 1;
				// continue...
			}
			' ' | '\t' | '\r' => {
				// continue...
			}
			'a'..='z' | 'A'..='Z' | '_' => {
				let mut word = String::from(ch);
				idx += 1;
				while idx < code.len() {
					ch = code[idx];
					match ch {
						'a'..='z' | 'A'..='Z' | '_' => {
							column += 1;
							word.push(ch);
						}
						_ => {
							idx -= 1;
							break;
						}
					}
					idx += 1;
				}
				match word.as_str() {
					"fun" => {}
					_ => {
						SyntaxError!("unexpected identifier {word:?} here")
					}
				}
			}
			_ => {
				SyntaxError!("unknown or unexpected char {ch:?}\n    at {id}:{line}:{column}");
			}
		}
		idx += 1;
		column += 1;
	}

	Module {}
}