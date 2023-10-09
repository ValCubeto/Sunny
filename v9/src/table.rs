use std::fmt::Display;

use crate::debug;

const HORIZONTAL: &str = "─";
const VERTICAL: &str = "│";

const LEFT_UP: &str = "┌";
const RIGHT_UP: &str = "┐";
const LEFT_DOWN: &str = "└";
const RIGHT_DOWN: &str = "┘";

const HORIZONTAL_DOWN: &str = "┬";
const HORIZONTAL_UP: &str = "┴";
const CROSSED: &str = "┼";
const VERTICAL_RIGHT: &str = "├";
const VERTICAL_LEFT: &str = "┤";

pub fn print_table<const C: usize, const R: usize>(titles: [&str; C], rows: [[&str; C]; R]) {
  let table = String::from(RIGHT_UP);
  // let max_title_len = titles.iter().max_by_key(|s| s.len());
  let max_lens: Vec<usize> = Vec::with_capacity(C);
  for x in rows.iter() {}
  debug!(C, R);
}