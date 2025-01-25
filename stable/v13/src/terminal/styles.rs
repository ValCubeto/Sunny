use std::fmt;

macro_rules! def_consts {
  ($($name:ident = $value:expr ;)*) => {
    $(pub const $name: &str = $value;)*
  };
}

def_consts! {
  BOLD = "\u{1b}[1m";
  BOLD_END = "\u{1b}[22m";
  ITALIC = "\u{1b}[3m";
  ITALIC_END = "\u{1b}[23m";
  UNDERLINE = "\u{1b}[4m";
  UNDERLINE_END = "\u{1b}[24m";
  STRIKETHROUGH = "\u{1b}[9m";
  STRIKETHROUGH_END = "\u{1b}[29m";
  RED = "\u{1b}[31m";
  GREEN = "\u{1b}[32m";
  YELLOW = "\u{1b}[33m";
  BLUE = "\u{1b}[34m";
  MAGENTA = "\u{1b}[35m";
  CYAN = "\u{1b}[36m";
  BRIGHT_BLACK = "\u{1b}[90m";
  BRIGHT_RED = "\u{1b}[91m";
  BRIGHT_GREEN = "\u{1b}[92m";
  BRIGHT_YELLOW = "\u{1b}[93m";
  BRIGHT_BLUE = "\u{1b}[94m";
  BRIGHT_MAGENTA = "\u{1b}[95m";
  BRIGHT_CYAN = "\u{1b}[96m";
  FOREGROUND_END = "\u{1b}[39m";
  BG_RED = "\u{1b}[41m";
  BG_GREEN = "\u{1b}[42m";
  BG_YELLOW = "\u{1b}[43m";
  BG_BLUE = "\u{1b}[44m";
  BG_MAGENTA = "\u{1b}[45m";
  BG_CYAN = "\u{1b}[46m";
  BG_BRIGHT_BLACK = "\u{1b}[100m";
  BG_BRIGHT_RED = "\u{1b}[101m";
  BG_BRIGHT_GREEN = "\u{1b}[102m";
  BG_BRIGHT_YELLOW = "\u{1b}[103m";
  BG_BRIGHT_BLUE = "\u{1b}[104m";
  BG_BRIGHT_MAGENTA = "\u{1b}[105m";
  BG_BRIGHT_CYAN = "\u{1b}[106m";
  BG_END = "\u{1b}[49m";
}

/// #FFA500
const ORANGE: (u8, u8, u8) = (255, 165, 0);
/// #5050FF
const BLUEBERRY: (u8, u8, u8) = (80, 80, 255);
/// #FF14A5 (deep pink)
const PINK: (u8, u8, u8) = (255, 20, 165);

macro_rules! color_fn {
  ($(fn $fn_name:ident() => ($start:ident, $end:ident);)*) => {
    $(
      #[inline(always)]
      /// Returns a `String` with the given style to display.
      /// Does nothing if the --no-color flag is passed.
      fn $fn_name(&self) -> String {
        if unsafe { crate::COLORING } {
          format!("{}{}{}", $start, self, $end)
        } else {
          format!("{}", self)
        }
      }
    )*
  }
}
macro_rules! other_fn {
  ($(fn $fn_name:ident ( & $self:ident $(,)? $($arg:ident : $argtype:ty),* ) { $fn:expr })*) => {
    $(
      #[inline(always)]
      /// Applies the given style
      fn $fn_name(&$self, $($arg: $argtype),*) -> String {
        if unsafe { crate::COLORING } {
          $fn
        } else {
          format!("{}", $self)
        }
      }
    )*
  }
}

pub trait Stylize: fmt::Display {
  color_fn! {
    fn bold() => (BOLD, BOLD_END);
    fn italics() => (ITALIC, ITALIC_END);
    fn underline() => (UNDERLINE, UNDERLINE_END);
    fn strikethrough() => (STRIKETHROUGH, STRIKETHROUGH_END);
    fn red() => (RED, FOREGROUND_END);
    fn green() => (GREEN, FOREGROUND_END);
    fn yellow() => (YELLOW, FOREGROUND_END);
    fn blue() => (BLUE, FOREGROUND_END);
    fn magenta() => (MAGENTA, FOREGROUND_END);
    fn cyan() => (CYAN, FOREGROUND_END);
    fn bright_red() => (BRIGHT_RED, FOREGROUND_END);
    fn bright_green() => (BRIGHT_GREEN, FOREGROUND_END);
    fn bright_yellow() => (BRIGHT_YELLOW, FOREGROUND_END);
    fn bright_blue() => (BRIGHT_BLUE, FOREGROUND_END);
    fn bright_magenta() => (BRIGHT_MAGENTA, FOREGROUND_END);
    fn bright_cyan() => (BRIGHT_CYAN, FOREGROUND_END);
    fn bg_red() => (BG_RED, BG_END);
    fn bg_green() => (BG_GREEN, BG_END);
    fn bg_yellow() => (BG_YELLOW, BG_END);
    fn bg_blue() => (BG_BLUE, BG_END);
    fn bg_magenta() => (BG_MAGENTA, BG_END);
    fn bg_cyan() => (BG_CYAN, BG_END);
    fn bg_bright_red() => (BG_BRIGHT_RED, BG_END);
    fn bg_bright_green() => (BG_BRIGHT_GREEN, BG_END);
    fn bg_bright_yellow() => (BG_BRIGHT_YELLOW, BG_END);
    fn bg_bright_blue() => (BG_BRIGHT_BLUE, BG_END);
    fn bg_bright_magenta() => (BG_BRIGHT_MAGENTA, BG_END);
    fn bg_bright_cyan() => (BG_BRIGHT_CYAN, BG_END);
  }
  other_fn!(
    fn error(&self) {
      self.red().bold()
    }
    fn warning(&self) {
      self.yellow().bold()
    }
    fn info(&self) {
      self.cyan().bold()
    }
    fn success(&self) {
      self.green().bold()
    }
    fn deprecated(&self) {
      self.italics().strikethrough()
    }

    fn rgb(&self, rgb: (u8, u8, u8)) {
      format!("\u{1b}[38;2;{};{};{}m{}\u{1b}[39m", rgb.0, rgb.1, rgb.2, self)
    }
    fn rgb_bg(&self, rgb: (u8, u8, u8)) {
      format!("\u{1b}[48;2;{};{};{}m{}\u{1b}[49m", rgb.0, rgb.1, rgb.2, self)
    }

    fn rgb_orange(&self) {
      self.rgb(ORANGE)
    }
    fn rgb_bg_orange(&self) {
      self.rgb_bg(ORANGE)
    }
    fn rgb_blueberry(&self) {
      self.rgb(BLUEBERRY)
    }
    fn rgb_bg_blueberry(&self) {
      self.rgb_bg(BLUEBERRY)
    }
    fn rgb_pink(&self) {
      self.rgb(PINK)
    }
    fn rgb_bg_pink(&self) {
      self.rgb_bg(PINK)
    }
  );
}

impl<T: fmt::Display> Stylize for T {}
