use std::process::exit;
use crate::{about::{ NAME, VERSION }, bold, table::print_table};

pub fn main() {
  print_table(["Command", "Description"], [
    ["help", "Prints this message"],
    ["version", format!("Prints the current {NAME} version").as_str()],
    ["repl", "Starts a new REPL session"]
  ]);
  println!("{}: {} [flags]* [command]", bold!("Usage"), NAME.to_lowercase());
  println!("┌─────────┬────────────────────────────┐");
  println!("│ Command │        Description         │");
  println!("├─────────┼────────────────────────────┤");
  println!("│ help    │ Prints this message        │");
  println!("│ version │ Prints the current {NAME} version │");
  println!("│ repl    │ Starts a new REPL session  │");
  println!("└─────────┴────────────────────────────┘");
  println!();
  println!("{NAME} v{VERSION}");
  exit(0);
}