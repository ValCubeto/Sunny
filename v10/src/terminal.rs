use std::fmt::Display;

pub const BOLD:      &str = "\u{1b}[1m";
pub const BOLD_END:  &str = "\u{1b}[22m";

pub const RED:       &str = "\u{1b}[31m";
pub const GREEN:     &str = "\u{1b}[32m";
pub const YELLOW:    &str = "\u{1b}[33m";
pub const COLOR_END: &str = "\u{1b}[39m";

#[inline(always)]
pub fn red<T: Display>(string: T) -> String {
  format!("{RED}{string}{COLOR_END}")
}
#[inline(always)]
pub fn bold<T: Display>(string: T) -> String {
  format!("{BOLD}{string}{BOLD_END}")
}
