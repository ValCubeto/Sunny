use std::ops::Div;

use crate::debug;

const HORIZONTAL: char = '─';
const VERTICAL: char = '│';

const LEFT_UP: char = '┌';
const RIGHT_UP: char = '┐';
const LEFT_DOWN: char = '└';
const RIGHT_DOWN: char = '┘';

const HORIZONTAL_DOWN: char = '┬';
const HORIZONTAL_UP: char = '┴';
const CROSSED: char = '┼';
const VERTICAL_RIGHT: char = '├';
const VERTICAL_LEFT: char = '┤';

pub fn print_table<const C: usize, const R: usize>(titles: [&str; C], rows: [[&str; C]; R]) {
  let mut table = String::from(LEFT_UP);

  let mut max_lens: Vec<usize> = Vec::with_capacity(C);
  for column in titles.iter() {
    max_lens.push(column.len());
  }
  for row in rows.iter() {
    for (i, column) in row.iter().enumerate() {
      if column.len() > max_lens[i] {
        max_lens[i] = column.len();
      }
    }
  }
  let mut iter = max_lens.iter();
  let len = iter.next().unwrap();
  table.push_str(HORIZONTAL.to_string().repeat(len + 2).as_str());
  for column_len in iter {
    table.push(HORIZONTAL_DOWN);
    table.push_str(HORIZONTAL.to_string().repeat(column_len + 2).as_str());
  }
  debug!(table);
}