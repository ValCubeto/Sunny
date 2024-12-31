#[derive(Debug)]
pub enum Number {
  Int(String),
  Float(String, String),
  Hex(String),
  Bin(String),
}

impl std::fmt::Display for Number {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Number::Int(int) => write!(f, "{int}"),
      Number::Float(int, frac) => write!(f, "{int}.{frac}"),
      Number::Hex(hex) => write!(f, "0x{hex}"),
      Number::Bin(bin) => write!(f, "0b{bin}"),
    }
  }
}
