#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

const HORIZONTAL: char = '─';
const VERTICAL: char = '│';

const LEFT_UP: char = '┌';
const RIGHT_UP: char = '┐';
const LEFT_DOWN: char = '└';
const RIGHT_DOWN: char = '┘';

const HORIZONTAL_DOWN: char = '┬';
const HORIZONTAL_UP: char = '┴';
const INTERSECTION: char = '┼';
const VERTICAL_RIGHT: char = '├';
const VERTICAL_LEFT: char = '┤';

// TODO: liberar algo de memoria antes de insertar slices para mejorar un poco el rendimiento
pub fn print_table(titles: &[&str], rows: &[&[&str]]) {
  let C = titles.len();
  assert!(C > 0);
  let R = rows[0].len();

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
  table.push_str(HORIZONTAL.to_string().repeat(iter.next().unwrap() + 2).as_str());
  for column_len in iter {
    table.push(HORIZONTAL_DOWN);
    table.push_str(HORIZONTAL.to_string().repeat(column_len + 2).as_str());
  }
  table.push(RIGHT_UP);


  table.push_str(LINE_ENDING);


  table.push(VERTICAL);
  for (i, len) in max_lens.iter().enumerate() {
    let title = titles[i];
    let half = (len - title.len()) as f64 / 2.0;
    table.push_str(" ".repeat(half.floor() as usize + 1).as_str());
    table.push_str(title);
    table.push_str(" ".repeat(half.ceil() as usize + 1).as_str());
    table.push(VERTICAL);
  }

  table.push_str(LINE_ENDING);
  table.push(VERTICAL_RIGHT);
  let mut iter = max_lens.iter();
  table.push_str(HORIZONTAL.to_string().repeat(iter.next().unwrap() + 2).as_str());
  for column_len in iter {
    table.push(INTERSECTION);
    table.push_str(HORIZONTAL.to_string().repeat(column_len + 2).as_str());
  }
  table.push(VERTICAL_LEFT);


  for row in rows.iter() {
    table.push_str(LINE_ENDING);
    table.push(VERTICAL);
    for (i, column) in row.iter().enumerate() {
      let len = max_lens[i];
      let half = (len - column.len()) as f64 / 2.0;
      table.push_str(" ".repeat(half.floor() as usize + 1).as_str());
      table.push_str(column);
      table.push_str(" ".repeat(half.ceil() as usize + 1).as_str());
      table.push(VERTICAL);
    }
  }


  table.push_str(LINE_ENDING);


  table.push(LEFT_DOWN);
  let mut iter = max_lens.iter();
  table.push_str(HORIZONTAL.to_string().repeat(iter.next().unwrap() + 2).as_str());
  for column_len in iter {
    table.push(HORIZONTAL_UP);
    table.push_str(HORIZONTAL.to_string().repeat(column_len + 2).as_str());
  }
  table.push(RIGHT_DOWN);


  println!("{table}");
}