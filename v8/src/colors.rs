pub const BOLD: &str = "\u{1b}[1m";
pub const BOLD_END: &str = "\u{1b}[22m";

pub const RED: &str = "\u{1b}[31m";
pub const YELLOW: &str = "\u{1b}[33m";
pub const COLOR_END: &str = "\u{1b}[39m";

#[macro_export]
macro_rules! display_red {
	($value:expr) => {
		format!("{}{}{}", $crate::colors::RED, $value, $crate::colors::COLOR_END)
	};
}