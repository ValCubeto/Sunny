use hashbrown::HashMap;

use crate::strings::LANG;
use crate::terminal::{ Align, Stylize, Table };

pub fn help() {
  println!("{}'s runtime and package manager", LANG.orange().bold());
  println!();
  println!("{}", "Usage:".bold());
  println!("  {} [command] [input] [args]", LANG.to_lowercase());
  let titles: HashMap<_, _> = HashMap::from([
    ("Command", "Description".to_owned()),
  ]);
  let commands = HashMap::from([
    ("help", "Prints this message".to_owned()),
    ("version", format!("Print your current version of {LANG}")),
    ("run", format!("Run a {LANG} file")),
    ("init", format!("Create a new {LANG} proyect"))
  ]);
  let commands = Table::new(None, &commands)
    .left_modifier(Stylize::note);
  Table::new(None, &titles)
    .align(Align::Center)
    .print_with(&commands);
  // println!();
  // println!("{}", "Flags:".bold());
  println!("{}: use `--` to stop parsing flags", "Note".note());
  println!();
}
