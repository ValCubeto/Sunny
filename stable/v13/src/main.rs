#[macro_use]
mod imports;
#[macro_use]
mod terminal;
mod strings;
mod args;
mod commands;
mod eval;
mod ctx;

pub static mut COLORING: bool = true;
pub static mut DEBUG: bool = false;

fn main() {
  let args = args::parse();
  if args.flags.contains_key("no-color") {
    unsafe {
      COLORING = false;
    }
  }
  if args.flags.contains_key("debug") {
    unsafe {
      DEBUG = true;
    }
  }
  debug_msg!("{args:?}");
  commands::run_cmd(args);
}
