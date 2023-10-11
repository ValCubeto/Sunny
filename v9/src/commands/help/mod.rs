use std::process::exit;
use crate::{
  about::{ NAME, VERSION },
  bold,
  table::print_table
};

pub fn main() {
  println!("{}: {} [flags]* [command]", bold!("Usage"), NAME.to_lowercase());
  print_table(
    ["Command", "Description"],
    [
      ["help", "Prints this message"],
      ["version", format!("Prints the current {NAME} version").as_str()],
      ["repl", "Starts a new REPL session"],
      ["run", "asd"]
    ]
  );
  println!();
  println!("{NAME} v{VERSION}");
  exit(0);
}