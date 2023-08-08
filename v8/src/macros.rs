#[macro_export]
macro_rules! error {
	($error_name:expr, $($arg:expr),*) => {{
		print!("{}:", $crate::display_red!("InternalError"));
		println!($($arg),*);
		exit(1);
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
