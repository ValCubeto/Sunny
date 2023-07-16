#[allow(unused)]
macro_rules! do_error {
	($name:expr, $($args:expr),+) => {{
		print!("{}: ", crate::colors::error($name));
		println!($($args),+);
		std::process::exit(1);
	}};
}

#[allow(unused)]
macro_rules! do_error_stack {
	($ctx:expr, $name:expr, $($args:expr),+) => {{
		print!("{}: ", crate::colors::error($name));
		println!($($args),+);
		println!("    at {:?}:{}:{}", $ctx.id, $ctx.line, $ctx.column);
		println!("    at {}:{}:{}", file!(), line!(), column!());
		std::process::exit(1);
	}};
}

#[allow(unused)]
macro_rules! InternalError {
	($($args:expr),+) => {{
		crate::errors::do_error!("InternalError", $($args),+);
	}};
}

#[allow(unused)]
macro_rules! ArgumentError {
	($($args:expr),+) => {{
		crate::errors::do_error!("ArgumentError", $($args),+);
	}};
}

#[allow(unused)]
macro_rules! LoadError {
	($($args:expr),+) => {{
		crate::errors::do_error!("LoadError", $($args),+);
	}};
}

#[allow(unused)]
macro_rules! SyntaxError {
	($ctx:expr, $($args:expr),+) => {{
		crate::errors::do_error_stack!($ctx, "SyntaxError", $($args),+);
	}};
}

#[allow(unused)]
macro_rules! ReferenceError {
	($ctx:expr, $($args:expr),+) => {{
		crate::errors::do_error_stack!($ctx, "ReferenceError", $($args),+);
	}};
}

#[allow(unused)]
macro_rules! TypeError {
	($ctx:expr, $($args:expr),+) => {{
		crate::errors::do_error_stack!($ctx, "TypeError", $($args),+);
	}};
}

#[allow(unused)]
pub(crate) use {
	do_error,
	do_error_stack,
	InternalError,
	ArgumentError,
	LoadError,
	SyntaxError,
	ReferenceError,
	TypeError
};