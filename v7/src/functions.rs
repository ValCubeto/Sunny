use crate::{ context::Context,
	id::Id,
	syntax_error,
	statments::Statment,
	// numbers::collect_num,
	expressions::parse_expr,
	arguments::Arguments,
	values::Value };

pub fn parse_function(ctx: &mut Context, name: Id, is_async: bool) -> Function {
	let mut function = Function::new(name, is_async);

	ctx.go();

	if ctx.current == '<' {
		ctx.next_char();
		ctx.go();
		'collect: loop {
			if ctx.current == '>' {
				ctx.next_char();
				break 'collect;
			}
			syntax_error!("function generics to do"; ctx);
			// 'sub: {
			// }
			// ctx.next_char();
			// ctx.go();
		}
	}

	ctx.go();

	if ctx.current != '(' {
		syntax_error!("expected '(', found {:?}", ctx.current; ctx);
	}
	ctx.next_char();
	ctx.go();
	'collect: loop {
		if ctx.current == ')' {
			ctx.next_char();
			break 'collect;
		}
		syntax_error!("function parameters to do"; ctx);
		// 'sub: {
		// }
		// ctx.next_char();
		// ctx.go();
	}
	ctx.go();

	if ctx.current == '-' {
		ctx.next_char();
		if ctx.current != '>' {
			syntax_error!("expected '>' (to complete '->'), found {:?}", ctx.current; ctx);
		}
		ctx.next_char();
		ctx.go();
		syntax_error!("functions' return type to do"; ctx);
	}

	if ctx.current != '{' {
		syntax_error!("expected '{{', found {:?}", ctx.current; ctx);
	}
	ctx.next_char();
	ctx.go();

	'collect: loop {
		if ctx.current == '}' {
			break 'collect;
		}
		'sub: {
			if ctx.is_valid_id() {
				let word = ctx.collect_word();
				ctx.go();
				// match word
				match ctx.current {
					'=' => {
						ctx.next_char();
						let expr = parse_expr(ctx);
						function.body.push(Statment::Assignment {
							id: word,
							mutable: false,
							expr
						})
					}
					'(' => {
						ctx.next_char();
						loop {
							ctx.go();
							if ctx.current == ')' {
								ctx.next_char();
								break;
							}
							#[allow(unused)]
							let expr = parse_expr(ctx);
							// ctx.next_char();
						}
					}
					_ => syntax_error!("unexpected character {:?}", ctx.current; ctx)
				}
				break 'sub;
			}
			// if ctx.current.is_ascii_digit() {
			// 	let number = collect_num(ctx);
			// 	dbg!(&number);
			// 	break 'sub
			// }

			// match ctx.current {
			// 	// '+' => {}
			// 	_ => 
			// }
			syntax_error!("unexpected character {:?}", ctx.current; ctx)
		}
		ctx.next_char();
		ctx.go();
	}

	function
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
	pub name: Id,
	pub is_async: bool,
	pub body: Vec<Statment>
}

impl Function {
	pub fn new(name: Id, is_async: bool) -> Self {
		Function {
			name,
			is_async,
			body: Vec::new()
		}
	}
}

#[allow(unused)]
#[derive(Debug)]
pub struct Error {
	name: Id,
	description: Id
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(unused)]
pub enum FunctionValue {
	Builtin(fn(Arguments) -> Result<Value, Error>), // Result<Value, &str>
	Defined(Function)
}

impl FunctionValue {
	pub fn call(&self, args: Arguments) -> Result<Value, Error> {
		match self {
			Self::Builtin(func) => func(args),
			_ => todo!()
		}
	}
	pub fn unwrap_defined(&self) -> &Function {
		match self {
			Self::Defined(func) => func,
			_ => panic!("unwrapping a builtin function")
		}
	}
}

