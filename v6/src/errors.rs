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

macro_rules! LoadError {
	($($args:expr),+) => {{
		crate::errors::Error!("LoadError", $($args),+);
	}};
} pub(crate) use LoadError;

macro_rules! SyntaxError {
	($($args:expr),+) => {{
		crate::errors::Error!("SyntaxError", $($args),+);
	}};
} pub(crate) use SyntaxError;
