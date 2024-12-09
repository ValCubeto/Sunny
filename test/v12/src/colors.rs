macro_rules! def_strings {
  ($($name:ident = $str:expr);* $(;)?) => {
    $(
      pub const $name: &str = $str;
    )*
  }
}

def_strings! {
  BOLD     = "\u{1b}[1m";
  BOLD_END = "\u{1b}[22m";

  RED      = "\u{1b}[31m";
  GREEN    = "\u{1b}[32m";
  YELLOW   = "\u{1b}[33m";
  COL_END  = "\u{1b}[39m";
}

macro_rules! def_formatters {
  ($(fn $name:ident($arg:ident) => $fmt:expr;)* $(;)?) => {
    $(
      pub fn $name<T: ::std::fmt::Display>($arg: T) -> String {
        format!($fmt)
      }
    )*
  };
}

def_formatters! {
  fn bold(text) => "{BOLD}{text}{BOLD_END}";
  fn red(text) => "{RED}{text}{COL_END}";
  fn green(text) => "{GREEN}{text}{COL_END}";
  fn yellow(text) => "{YELLOW}{text}{COL_END}";
}
