extern crate rand;
extern crate num_bigint;
extern crate bigdecimal;

mod colors;
mod about;
mod macros;
mod aliases;
mod values;
mod context;
mod args;
mod commands;
mod table;

use args::parse_args;

pub fn main() {
  parse_args();
}
