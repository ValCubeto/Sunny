mod types;
mod values;
mod structs;
mod instances;
mod enums;
mod tests;
mod variants;

use crate::tests::*;

fn main() {
  test_structs();
  test_enums();
}
