use crate::strings::{ LANG, VERSION };
use crate::terminal::Stylize;

pub fn help() {
  let help_txt = format!("{:7}", "help").blue().bold();
  let version_txt = format!("{:7}", "version").blue().bold();
  let run_txt = format!("{:7}", "run").blue().bold();
  let init_txt = format!("{:7}", "init").blue().bold();

  // TODO: create terminal::table()
  println!("{}'s runtime and package manager", format!("{LANG} v{VERSION}").cyan().bold());
  println!();
  println!("{}", "Usage:".bold());
  println!("  > {} [command] [input] [args]", LANG.to_lowercase());
  println!();
  println!("{}", "Commands:".bold());
  println!("  {} │ Prints this message", help_txt);
  println!("  {} │ Print your current version of {LANG}", version_txt);
  println!("  {} │ Run a {LANG} file", run_txt);
  println!("  {} │ Create a new {LANG} proyect", init_txt);
  // println!();
  // println!("{}", "Flags:".bold());
  println!();
  println!("{}: use `--` to stop parsing flags", "Tip".bold());
}
