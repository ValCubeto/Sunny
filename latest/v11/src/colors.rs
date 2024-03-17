
pub const BOLD: &str = "\u{1b}[1m";
pub const BOLD_END: &str = "\u{1b}[22m";

pub const RED: &str = "\u{1b}[31m";
pub const GREEN: &str = "\u{1b}[32m";
pub const YELLOW: &str = "\u{1b}[33m";
pub const COL_END: &str = "\u{1b}[39m";

pub fn bold(text: &str) -> String {
  format!("{BOLD}{text}{BOLD_END}")
}

pub fn red(text: &str) -> String {
  format!("{RED}{text}{COL_END}")
}

pub fn green(text: &str) -> String {
  format!("{GREEN}{text}{COL_END}")
}

pub fn yellow(text: &str) -> String {
  format!("{YELLOW}{text}{COL_END}")
}
