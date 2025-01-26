use crate::args::ParsedArgs;

pub fn _test_(args: ParsedArgs) {
  match args.input.as_str() {
    "colors" => colors(),
    "table" => table(),
    other => eprintln!("no such test {other:?}")
  }
}

fn colors() {
  use crate::terminal::Stylize;
  println!("{}", "Bold".bold());
  println!("{}", "Italics".italics());
  println!("{}", "Underline".underline());
  println!("{}", "Strikethrough".strikethrough());
  println!("{}", "Console red".red());
  println!("{}", "Console green".green());
  println!("{}", "Console yellow".yellow());
  println!("{}", "Console blue".blue());
  println!("{}", "Console magenta".magenta());
  println!("{}", "Console cyan".cyan());
  println!("{}", "Console bright red".bright_red());
  println!("{}", "Console bright green".bright_green());
  println!("{}", "Console bright yellow".bright_yellow());
  println!("{}", "Console bright blue".bright_blue());
  println!("{}", "Console bright magenta".bright_magenta());
  println!("{}", "Console bright cyan".bright_cyan());
  println!("{}", "Console BG red".bg_red());
  println!("{}", "Console BG green".bg_green());
  println!("{}", "Console BG yellow".bg_yellow());
  println!("{}", "Console BG blue".bg_blue());
  println!("{}", "Console BG magenta".bg_magenta());
  println!("{}", "Console BG cyan".bg_cyan());
  println!("{}", "Console BG bright red".bg_bright_red());
  println!("{}", "Console BG bright green".bg_bright_green());
  println!("{}", "Console BG bright yellow".bg_bright_yellow());
  println!("{}", "Console BG bright blue".bg_bright_blue());
  println!("{}", "Console BG bright magenta".bg_bright_magenta());
  println!("{}", "Console BG bright cyan".bg_bright_cyan());
  println!("{}", "Error".error());
  println!("{}", "Warning".warning());
  println!("{}", "Info".info());
  println!("{}", "Success".success());
  println!("{}", "Deprecated".deprecated());
  println!("{}", "RGB orange".orange());
  println!("{}", "RGB BG orange".bg_orange());
  println!("{}", "RGB blueberry".blueberry());
  println!("{}", "RGB BG blueberry".bg_blueberry());
  println!("{}", "RGB pink".pink());
  println!("{}", "RGB BG pink".bg_pink());
}

fn table() {
  use hashbrown::HashMap;
  use crate::terminal::{ Stylize, Table, Align };

  let map1: HashMap<u8, u8> = HashMap::new();
  Table {
    title: Some("Empty"),
    map: &map1,
    apply: None,
    align: Align::Left
  }.print();

  let map2: HashMap<u8, u8> = HashMap::from([
    (12, 34),
    (5, 6)
  ]);
  Table {
    title: Some("Number"),
    map: &map2,
    apply: None,
    align: Align::Left
  }.print();

  let map3: HashMap<&str, &str> = HashMap::from([
    ("one", "uno"),
    ("two", "dos"),
    ("three", "tres")
  ]);
  /*
  ┌──────────────┐
  │   Stringgg   │
  | three | tres |
  */
  Table {
    title: Some("Stringsssssssssss"),
    map: &map3,
    apply: None,
    align: Align::Right
  }.print();

  let map4: HashMap<u32, &str> = HashMap::from([
    (11, "once"),
    (2, "dos"),
    (3, "tressssss")
  ]);
  Table {
    title: Some("Coloring"),
    map: &map4,
    apply: Some(|k, v| (k.orange(), v.blueberry())),
    align: Align::Center
  }.print();
}
