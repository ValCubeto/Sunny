mod colors;
mod about;
mod macros;
mod aliases;
mod values;
mod context;
mod args;
mod commands;

use std::mem::size_of;
use crate::{
  values::Value,
  args::parse_args,
};

pub fn main() {
  debug!(size_of::<Value>());
  parse_args();
}
