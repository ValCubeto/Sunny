use crate::context::Context;

pub fn collect_word(ctx: &mut Context) -> String {
	let mut word = String::new();
	while ctx.idx < ctx.char_count {
		match ctx.ch {
			'a'..='z' | 'A'..='Z' | '_' => {
				word.push(ctx.ch);
			}
			_ => {
				break;
			}
		}
		ctx.next_char();
	}
	word
}