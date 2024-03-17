#[macro_use]
  pub mod errors;
pub mod parse;
pub mod colors;

fn main() {
  let file_name = "main";
  let code = "\t\n
    const A = 1
  ";
  parse::parse_file(file_name, code);
}

// Global variables:
// - std
// - use std::numbers::{ u8, u16, u32, u64, i8, i16, i32, i64, f32, f64 };
// - typedef int = i32
// - typedef uint = u32
// - typedef float = f32
// - use std::veclab::Vec
// - use std::stringlab::String
// - use std::io::println
// - use std::process::exit
// - use std::debug::assert!
