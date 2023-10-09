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

  let mut max_lens: Vec<usize> = Vec::with_capacity(C);
  for row in rows.iter() {
    let mut max_len = row[0].len();
    for column in row.iter().skip(0) {
      let len = column.len();
      if len > max_len {
        max_len = len;
      }
    }
    max_lens.push(max_len);
  }
  debug!(max_lens);
}