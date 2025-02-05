use crate::args::ParsedArgs;

pub fn __test(args: ParsedArgs) {
  match args.input.as_str() {
    "colors" => colors(),
    "tables" => tables(),
    "termsize" => termsize(),
    other => eprintln!("No such test {other:?}")
  }
}

fn colors() {
  use crate::terminal::Stylize;
  println!("{}", "Bold".bold());
  println!("{}", "Italics".italics());
  println!("{}", "Underline".underline());
  println!("{}", "Strikethrough".strikethrough());
  println!();
  println!("{}", "Console black".black());
  println!("{}", "Console red".red());
  println!("{}", "Console green".green());
  println!("{}", "Console yellow".yellow());
  println!("{}", "Console blue".blue());
  println!("{}", "Console magenta".magenta());
  println!("{}", "Console cyan".cyan());
  println!("{}", "Console white".white());
  println!();
  println!("{}", "Console bright black".bright_black());
  println!("{}", "Console bright red".bright_red());
  println!("{}", "Console bright green".bright_green());
  println!("{}", "Console bright yellow".bright_yellow());
  println!("{}", "Console bright blue".bright_blue());
  println!("{}", "Console bright magenta".bright_magenta());
  println!("{}", "Console bright cyan".bright_cyan());
  println!("{}", "Console bright white".bright_white());
  println!();
  println!("{}", "Console BG black".bg_black());
  println!("{}", "Console BG red".bg_red());
  println!("{}", "Console BG green".bg_green());
  println!("{}", "Console BG yellow".bg_yellow());
  println!("{}", "Console BG blue".bg_blue());
  println!("{}", "Console BG magenta".bg_magenta());
  println!("{}", "Console BG cyan".bg_cyan());
  println!("{}", "Console BG white".bg_white());
  println!();
  println!("{}", "Console BG bright black".bg_bright_black());
  println!("{}", "Console BG bright red".bg_bright_red());
  println!("{}", "Console BG bright green".bg_bright_green());
  println!("{}", "Console BG bright yellow".bg_bright_yellow());
  println!("{}", "Console BG bright blue".bg_bright_blue());
  println!("{}", "Console BG bright magenta".bg_bright_magenta());
  println!("{}", "Console BG bright cyan".bg_bright_cyan());
  println!("{}", "Console BG bright white".bg_bright_white());
  println!();
  println!("{}", "RGB orange".orange());
  println!("{}", "RGB blueberry".blueberry());
  println!("{}", "RGB pink".pink());
  println!("{}", "RGB BG orange".bg_orange());
  println!("{}", "RGB BG blueberry".bg_blueberry());
  println!("{}", "RGB BG pink".bg_pink());
  println!();
  println!("{}", "Error".error());
  println!("{}", "Success".success());
  println!("{}", "Warning".warning());
  println!("{}", "Info".info());
  println!("{}", "Note".note());
  println!("{}", "Deprecated".deprecated());
}

fn tables() {
  use hashbrown::HashMap;
  use crate::terminal::{ Stylize, Table, Align };

  let map: HashMap<u8, u8> = HashMap::new();
  Table::new(None, &map)
    .print();

  let map: HashMap<&str, &str> = HashMap::from([
    ("no", "title")
  ]);
  Table::new(None, &map)
    .print();

  let map: HashMap<u8, u8> = HashMap::new();
  Table::new(Some("Empty table"), &map)
    .print();

  let map: HashMap<u8, u8> = HashMap::from([
    (12, 34),
    (5, 6),
    (100, 255)
  ]);
  Table::new(Some("Numbers"), &map)
    .print();

  let map: HashMap<&str, &str> = HashMap::from([
    ("one", "uno"),
    ("two", "dos"),
    ("three", "tres")
  ]);
  Table::new(Some("Strings"), &map)
    .align(Align::Right)
    .print();

  let map: HashMap<u32, &str> = HashMap::from([
    (2, "dos"),
    (11, "once"),
    (3333, "tres mil trescientos treinta y tres")
  ]);
  Table::new(Some("Coloring"), &map)
    .left_modifier(Stylize::orange)
    .right_modifier(Stylize::bg_blueberry)
    .align(Align::Center)
    .print();
}

fn termsize() {
  let (width, height) = match crate::terminal::stdout_size() {
    Some(size) => size,
    None => internal_err!("Couldn't get the size of the terminal")
  };
  println!("stdout size: {width}x{height}");
}
