use crate::args::ParsedArgs;

pub fn _test_(args: ParsedArgs) {
  match args.input.as_str() {
    "colors" => colors(),
    // "table"
    other => eprintln!("no such test {other:?}")
  }
}

fn colors() {
  use crate::terminal::Stylize;
  println!("{}", "Bold".bold());
  println!("{}", "Italics".italics());
  println!("{}", "Underline".underline());
  println!("{}", "Strikethrough".strikethrough());
  println!("{}", "Red".red());
  println!("{}", "Green".green());
  println!("{}", "Yellow".yellow());
  println!("{}", "Blue".blue());
  println!("{}", "Magenta".magenta());
  println!("{}", "Cyan".cyan());
  println!("{}", "Bright red".bright_red());
  println!("{}", "Bright green".bright_green());
  println!("{}", "Bright yellow".bright_yellow());
  println!("{}", "Bright blue".bright_blue());
  println!("{}", "Bright magenta".bright_magenta());
  println!("{}", "Bright cyan".bright_cyan());
  println!("{}", "BG red".bg_red());
  println!("{}", "BG green".bg_green());
  println!("{}", "BG yellow".bg_yellow());
  println!("{}", "BG blue".bg_blue());
  println!("{}", "BG magenta".bg_magenta());
  println!("{}", "BG cyan".bg_cyan());
  println!("{}", "BG bright red".bg_bright_red());
  println!("{}", "BG bright green".bg_bright_green());
  println!("{}", "BG bright yellow".bg_bright_yellow());
  println!("{}", "BG bright blue".bg_bright_blue());
  println!("{}", "BG bright magenta".bg_bright_magenta());
  println!("{}", "BG bright cyan".bg_bright_cyan());
  println!("{}", "Error".error());
  println!("{}", "Warning".warning());
  println!("{}", "Info".info());
  println!("{}", "Success".success());
  println!("{}", "Deprecated".deprecated());
  println!("{}", "RGB orange".rgb_orange());
  println!("{}", "RGB BG orange".rgb_bg_orange());
  println!("{}", "RGB blueberry".rgb_blueberry());
  println!("{}", "RGB BG blueberry".rgb_bg_blueberry());
  println!("{}", "RGB pink".rgb_pink());
  println!("{}", "RGB BG pink".rgb_bg_pink());
}
