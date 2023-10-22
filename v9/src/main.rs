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

use crate::table::print_table;

fn main() {
  let map = hashmap! {
    "hello" => 123_u8,
  };
  let mut entries: Vec<[&str; 2]> = vec![];
  print_table(["Key", "Value"], entries.as_slice().into());
  todo!();
  parse_args();
}
