use std::process::exit;
use crate::{
  about::{ NAME, VERSION },
  table::print_table,
  bold,
};

pub fn main() {
  println!("{}: {} [flags]* [command]", bold!("Usage"), NAME.to_lowercase());
  print_table(
    vec!["Command", "Description"],
    vec![
      vec!["help", "Prints this message"],
      vec!["version", format!("Prints the current {NAME} version").as_str()],
      vec!["repl", "Starts a new REPL session"],
      vec!["run", "asd"]
    ]
  );
  println!();
  println!("{NAME} v{VERSION}");
  exit(0);
}