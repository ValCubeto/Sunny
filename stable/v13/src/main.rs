use std::sync::atomic::{ AtomicBool, Ordering::Relaxed };

#[macro_use]
mod imports;
#[macro_use]
mod terminal;
mod strings;
mod args;
mod commands;
mod eval;
mod ctx;

pub static COLORING: AtomicBool = AtomicBool::new(true);
pub static DEBUG: AtomicBool = AtomicBool::new(false);

fn main() {
  let args = args::parse();
  if args.flags.contains_key("no-color") {
    COLORING.store(false, Relaxed);
  }
  if args.flags.contains_key("debug") {
    DEBUG.store(true, Relaxed);
  }

  debug_msg!("{args:?}");
  commands::run_cmd(args);
}
