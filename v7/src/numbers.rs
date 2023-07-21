use crate::{context::Context, errors::SyntaxError};

pub fn collect_num(ctx: &mut Context) -> (String, NumberKind) {
	let mut num = String::from(ctx.current);
	ctx.next_char();
	loop {
		if !ctx.current.is_ascii_digit() {
			break;
		}
		num.push(ctx.current);
		ctx.next_char();
	}
	(num, NumberKind::Int)
}
/* if ctx.current != '0' {
	ctx.next_char();
	// let has_dot = false;
	loop {
		if ctx.current == 'e' {
			SyntaxError!("not implemented");
			// ctx.next_char();
			// if !ctx.current.is_ascii_digit() {
			// 	SyntaxError!(ctx, "expected a digit, got {:?}", ctx.current);
			// }
			// let mut exponent = String::from(ctx.current);
		}
		if !ctx.current.is_ascii_digit() {
			break;
		}
		num.push(ctx.current);
	}
	return num;
}
ctx.next_char();
if ctx.current == 'b' {
	SyntaxError!(ctx, "not implemented");
	// ctx.next_char();
	// while ctx.current.is_digit(2) {
	// 	num.push(ctx.current);
	// 	ctx.next_char();
	// }
	// ctx.debug();
	// return num;
}
if ctx.current == 'x' {
	SyntaxError!(ctx, "not implemented");
}
if !ctx.current.is_ascii_digit() {
	loop {}
} */

pub enum NumberKind {
	Int, Float
}