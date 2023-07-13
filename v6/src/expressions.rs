use crate::{context::Context, errors::SyntaxError};

pub struct Expression {
	items: Vec<String>
}

pub fn parse_expr(ctx: &mut Context) -> Expression {
	ctx.ignore_spaces();
	let mut expression = Expression {
		items: Vec::new()
	};

	while ctx.idx < ctx.char_count {
		if ctx.ch.is_alphabetic() {
			ctx.next_char();
			continue;
		}
		if ctx.ch.is_numeric() {
			ctx.next_char();
			continue;
		}
		match ctx.ch {
			'\'' | '"' => {
				let string = ctx.collect_string();
				todo!("asdasdsad");
			}
			_ => {
				SyntaxError!(ctx, "que {:?}", ctx.ch)
			}
		};
		ctx.next_char();
	}

	expression
}