#[macro_use]
  pub mod errors;
pub mod parse;
pub mod colors;

fn main() {
  let file_name = "main";
  let code = "\t\n
    const A = 1
  ";
  parse::parse_file(file_name, code);
}
