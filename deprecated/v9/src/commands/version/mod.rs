use std::process::exit;
use crate::about::VERSION;

pub fn main() {
  println!("{VERSION}");
  exit(0);
}