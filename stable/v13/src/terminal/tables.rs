use std::fmt;
use chrono::format;
use hashbrown::HashMap;

/*
+---------------+
|     Title     |
+-------+-------+
| one   | uno   |
| two   | dos   |
| three | tres  |
+-------+-------+

+-------+----------+
| uno   | one      |
| dos   | two      |
| tres  | threeeee |
+-------+----------+
| one   | uno      |
| two   | dos      |
| three | tres     |
+-------+----------+

*/

macro_rules! def_consts {
  ($($name:ident = $value:expr ;)*) => {
    $(pub const $name: &str = $value;)*
  };
}

def_consts! {
  UP_RIGHT = "┐";
  UP_LEFT = "┌";
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
impl fmt::Display for Align {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Align::Left => write!(f, "left"),
      Align::Center => write!(f, "center"),
      Align::Right => write!(f, "right")
    }
  }
}

type ApplyFn = fn (&String) -> String;

pub struct Table<'a, K, V>
where
  K: fmt::Display,
  V: fmt::Display
{
  pub title: Option<&'a str>,
  pub map: &'a HashMap<K, V>,
  /// To apply a style that doesn't change the length of the string
  pub left_modifier: Option<ApplyFn>,
  pub right_modifier: Option<ApplyFn>,
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
  pub fn new(title: Option<&'a str>, map: &'a HashMap<K, V>) -> Self {
    Self {
      title,
      map,
      left_modifier: None,
      right_modifier: None,
      align: Align::Left
    }
  }
  pub fn left_modifier(mut self, left_modifier: ApplyFn) -> Self {
    self.left_modifier = Some(left_modifier);
    self
  }
  pub fn right_modifier(mut self, right_modifier: ApplyFn) -> Self {
    self.right_modifier = Some(right_modifier);
    self
  }
  pub fn align(mut self, align: Align) -> Self {
    self.align = align;
    self
  }
  pub fn print(&self) {
    let entries: Vec<(String, String)> = self.map.iter()
      .map(|(k, v)| (k.to_string(), v.to_string()))
      .collect();
    let mut len_left = entries.iter().map(|(k, _)| k.len()).max().unwrap_or(1);
    let mut len_right = entries.iter().map(|(_, v)| v.len()).max().unwrap_or(1);
    let mut len = len_left + len_right + 3;

    if let Some(title) = self.title {
      if title.len() > len {
        len = title.len();
        len_left = len / 2 - 1;
        len_right = len / 2 - 1;
        if title.len() % 2 == 0 {
          len_right -= 1;
        }
      }
      println!(
        "{}{}{}",
        UP_LEFT,
        HORIZONTAL.repeat(len + 2),
        UP_RIGHT,
      );
      println!(
        "{} {} {}",
        VERTICAL,
        pad(title, len, Align::Center),
        VERTICAL
      );
      println!(
        "{}{}{}{}{}",
        VERTICAL_LEFT,
        HORIZONTAL.repeat(len_left + 2),
        HORIZONTAL_UP,
        HORIZONTAL.repeat(len_right + 2),
        VERTICAL_RIGHT
      );
    } else {
      println!(
        "{}{}{}{}{}",
        UP_LEFT,
        HORIZONTAL.repeat(len_left + 2),
        HORIZONTAL_UP,
        HORIZONTAL.repeat(len_right + 2),
        UP_RIGHT
      );
    }

    for (mut k, mut v) in entries {
      let mut left = pad(&k, len_left, self.align);
      let mut right = pad(&v, len_right, self.align);
      // I don't feel like optimizing this
      if let Some(left_modifier) = self.left_modifier {
        left = left.replace(&k, &left_modifier(&k));
      }
      if let Some(right_modifier) = self.right_modifier {
        right = right.replace(&v, &right_modifier(&v));
      }
      println!(
        "{} {} {} {} {}",
        VERTICAL,
        left,
        VERTICAL,
        right,
        VERTICAL
      )
    }

    println!(
      "{}{}{}{}{}",
      DOWN_LEFT,
      HORIZONTAL.repeat(len_left + 2),
      HORIZONTAL_DOWN,
      HORIZONTAL.repeat(len_right + 2),
      DOWN_RIGHT
    );
  }
  pub fn print_with(&self, other: &Table<'a, K, V>) {
    let entries: Vec<(String, String)> = self.map.iter()
      .map(|(k, v)| (k.to_string(), v.to_string()))
      .collect();
    let other_entries: Vec<(String, String)> = other.map.iter()
      .map(|(k, v)| (k.to_string(), v.to_string()))
      .collect();
    let mut all_entries = entries.iter().chain(other_entries.iter());
    let mut len_left = all_entries.clone().map(|(k, _)| k.len()).max().unwrap_or(1);
    let mut len_right = all_entries.map(|(_, v)| v.len()).max().unwrap_or(1);
    let mut len = len_left + len_right + 3;

    if let Some(title) = self.title {
      if title.len() > len {
        len = title.len();
        len_left = len / 2 - 1;
        len_right = len / 2 - 1;
        if title.len() % 2 == 0 {
          len_right -= 1;
        }
      }
      println!(
        "{}{}{}",
        UP_LEFT,
        HORIZONTAL.repeat(len + 2),
        UP_RIGHT,
      );
      println!(
        "{} {} {}",
        VERTICAL,
        pad(title, len, Align::Center),
        VERTICAL
      );
      println!(
        "{}{}{}{}{}",
        VERTICAL_LEFT,
        HORIZONTAL.repeat(len_left + 2),
        HORIZONTAL_UP,
        HORIZONTAL.repeat(len_right + 2),
        VERTICAL_RIGHT
      );
    } else {
      println!(
        "{}{}{}{}{}",
        UP_LEFT,
        HORIZONTAL.repeat(len_left + 2),
        HORIZONTAL_UP,
        HORIZONTAL.repeat(len_right + 2),
        UP_RIGHT
      );
    }

    for (mut k, mut v) in entries {
      let mut left = pad(&k, len_left, self.align);
      let mut right = pad(&v, len_right, self.align);
      // I don't feel like optimizing this
      if let Some(left_modifier) = self.left_modifier {
        left = left.replace(&k, &left_modifier(&k));
      }
      if let Some(right_modifier) = self.right_modifier {
        right = right.replace(&v, &right_modifier(&v));
      }
      println!(
        "{} {} {} {} {}",
        VERTICAL,
        left,
        VERTICAL,
        right,
        VERTICAL
      )
    }

    println!(
      "{}{}{}{}{}",
      VERTICAL_LEFT,
      HORIZONTAL.repeat(len_left + 2),
      INTERSECTION,
      HORIZONTAL.repeat(len_right + 2),
      VERTICAL_RIGHT
    );

    for (mut k, mut v) in other_entries {
      let mut left = pad(&k, len_left, other.align);
      let mut right = pad(&v, len_right, other.align);
      // I don't feel like optimizing this
      if let Some(left_modifier) = other.left_modifier {
        left = left.replace(&k, &left_modifier(&k));
      }
      if let Some(right_modifier) = other.right_modifier {
        right = right.replace(&v, &right_modifier(&v));
      }
      println!(
        "{} {} {} {} {}",
        VERTICAL,
        left,
        VERTICAL,
        right,
        VERTICAL
      )
    }

    println!(
      "{}{}{}{}{}",
      DOWN_LEFT,
      HORIZONTAL.repeat(len_left + 2),
      HORIZONTAL_DOWN,
      HORIZONTAL.repeat(len_right + 2),
      DOWN_RIGHT
    );
  }
}


fn pad(string: &str, len: usize, align: Align) -> String {
  match align {
    Align::Left   => format!("{string: <len$}"),
    Align::Center => format!("{string: ^len$}"),
    Align::Right  => format!("{string: >len$}")
  }
}
