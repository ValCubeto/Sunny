mod strings;
mod args;
mod commands;
mod imports;
mod terminal;

fn main() {
  // NOTE: args[0] is the name of the executable
  let args = args::parse();
  debug!(args);

  commands::run_cmd(args);
}
