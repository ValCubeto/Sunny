use std::fmt;
use chrono::format;
use hashbrown::HashMap;

macro_rules! def_consts {
  ($($name:ident = $value:expr ;)*) => {
    $(pub const $name: &str = $value;)*
  };
}

def_consts! {
  UP_LEFT = "┐";
  UP_RIGHT = "┌";
  DOWN_LEFT = "└";
  DOWN_RIGHT = "┘";
  HORIZONTAL = "─";
  HORIZONTAL_UP = "┬";
  HORIZONTAL_DOWN = "┴";
  VERTICAL = "│";
  VERTICAL_LEFT = "├";
  VERTICAL_RIGHT = "┤";
  INTERSECTION = "┼";
  SPACE = " ";
}

#[derive(Debug, Clone, Copy)]
pub enum Align {
  Left,
  Center,
  Right
}

type ApplyFn = fn (String, String) -> (String, String);

pub struct Table<'a, K, V>
where
  K: fmt::Display,
  V: fmt::Display
{
  pub title: Option<&'a str>,
  pub map: &'a HashMap<K, V>,
  /// To apply a style that doesn't change the length of the string
  pub apply: Option<ApplyFn>,
  pub align: Align
}

fn pad_even(string: &str, ch: &str) -> String {
  let mut string = string.to_string();
  if string.len() % 2 == 0 {
    string.push_str(ch);
  }
  string
}

impl<'a, K, V> Table<'a, K, V>
where
  K: fmt::Display,
  V: fmt::Display
{
  pub fn print(&self) {
    let entries: Vec<(String, String)> = self.map.iter()
      .map(|(k, v)| (k.to_string(), v.to_string()))
      .collect();
    let len_left = entries.iter().map(|(k, _)| k.len()).max().unwrap_or(1);
    let len_right = entries.iter().map(|(_, v)| v.len()).max().unwrap_or(1);
    let mut len = len_left + len_right + 1;
    todo!("i give up");
  }
}

fn pad(string: &str, len: usize, align: Align) -> String {
  match align {
    Align::Left => format!("{string:->len$}"),
    Align::Center => {
      // align to center using rust fmt
      format!("{string:-^len$}")
    },
    Align::Right => format!("{string:-<len$}")
  }
}
