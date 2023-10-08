use std::process::exit;
use crate::about::{ NAME, VERSION };

pub fn main() {
  println!("{NAME} v{VERSION}");
  println!("Usage: {} [flags]* [command]", NAME.to_lowercase());
  println!();
  println!("help        Prints this message");
  println!("version     Prints the current {NAME} version");
  println!("repl        Starts a new REPL session");
  exit(0);
}