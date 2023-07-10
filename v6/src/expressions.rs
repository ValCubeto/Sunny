use crate::context::Context;

pub struct Expression {
	items: Vec<String>
}

pub fn parse_expr(ctx: &mut Context) -> Expression {
	ctx.ignore_spaces();
	let mut expression = Expression {
		items: Vec::new()
	};

	while ctx.idx < ctx.char_count {
		ctx.next_char();
		match ctx.ch {};
	}

	expression
}