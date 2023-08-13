#[macro_export]
macro_rules! error {
	($error_name:expr, $($arg:expr),*) => {{
		print!("{}: ", $crate::display_bold!($crate::display_red!($error_name)));
		println!($($arg),*);
		std::process::exit(1);
	}};
	($error_name:expr, $($arg:expr),*; $ctx:expr) => {{
		print!("{}: ", $crate::display_bold!($crate::display_red!($error_name)));
		println!($($arg),*);
		println!("    at {}:{}:{}", $ctx.id, $ctx.line, $ctx.column);
		std::process::exit(1);
	}};
}

#[macro_export]
macro_rules! internal_error {
	($($arg:expr),*) => { $crate::error!("InternalError", $($arg),*) }
}

#[macro_export]
macro_rules! argument_error {
	($($arg:expr),*) => { $crate::error!("ArgumentError", $($arg),*) }
}

#[macro_export]
macro_rules! load_error {
	($($arg:expr),*) => { $crate::error!("LoadError", $($arg),*) }
}

#[macro_export]
macro_rules! syntax_error {
	($($arg:expr),*; $ctx:expr) => { $crate::error!("LoadError", $($arg),*; $ctx) }
}
