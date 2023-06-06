macro_rules! Warning {
	($($args:expr),+) => {{
		eprint!("{}: ", crate::colors::warning("Warning"));
		eprintln!($($args),+);
	}};
} pub(crate) use Warning;

macro_rules! Error {
	($name:expr, $($args:expr),+) => {{
		eprint!("{}: ", crate::colors::error($name));
		eprintln!($($args),+);
		std::process::exit(1);
	}};
} pub(crate) use Error;

macro_rules! InternalError {
	($($args:expr),+) => {{
		crate::errors::Error!("InternalError", $($args),+);
	}};
} pub(crate) use InternalError;

macro_rules! ArgumentError {
	($($args:expr),+) => {{
		crate::errors::Error!("ArgumentError", $($args),+);
	}};
} pub(crate) use ArgumentError;
