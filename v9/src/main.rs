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
mod util;

use args::parse_args;

use crate::table::print_table;

fn main() {
  let hashmap = hashmap! {
    "hello" => 123_u8,
  };

  let mut entries: Vec<(&str, u8)> = util::map_entries(hashmap);
  let mut result: Vec<&[&str]> = Vec::with_capacity(entries.len());
  for &(key, value) in entries.iter() {
    result.push(&[key, &value.to_string()])
  }
  debug!("{result:?}");
  print_table(vec!["Key", "Value"].as_slice(), &result);
  todo!();
  parse_args();
}
