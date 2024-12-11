#[macro_use]
mod imports;
#[macro_use]
mod terminal;
mod strings;
mod args;
mod commands;
mod eval;
mod ctx;

fn main() {
  let args = args::parse();
  debug!(args);
  commands::run_cmd(args);
}
