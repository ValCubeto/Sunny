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
  let hashmap = hashmap! {
    "hello" => 123_u8,
  };

    // Convert the HashMap to a Vec<Vec<String>>
    let mut slices: Vec<Vec<String>> = hashmap
        .into_iter()
        .map(|(key, value)| vec![key.to_string(), value.to_string()])
        .collect();

    // Convert strings to static slices
    let static_slices: Vec<Vec<&'static str>> = slices
        .iter_mut()
        .map(|inner_vec| {
            inner_vec
                .iter_mut()
                .map(|s| {
                    let static_ref: &'static str = Box::leak(s.into_boxed_str());
                    static_ref
                })
                .collect()
        })
        .collect();

    // Convert vector of Vec<&str> into &[&[&str]]
    let result: Vec<&[&str]> = static_slices.iter().map(|v| v.as_slice()).collect();


  debug!("{result:?}");
  print_table(vec!["Key", "Value"], result);
  todo!();
  parse_args();
}
