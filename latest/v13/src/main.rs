mod strings;
mod args;
mod commands;
mod imports;
mod terminal;
mod eval;
mod ctx;

fn main() {
  let args = args::parse();
  debug!(args);
  commands::run_cmd(args);
}
