pub enum Value {
  Boolean(bool),

  Uint8(u8),
  Uint16(u16),
  UInt32(u32),
  UInt64(u64),
  Usize(usize),

  Int8(i8),
  Int16(i16),
  Int32(i32),
  Int64(i64),
  Isize(isize),

  Float32(f32),
  Float64(f64),

  // Tuple(???),
  // Array(*const ()),
  // Vec(*mut ()),
  StringMut(String),
  // StringConst(*const str),
  // Char(char),

  // Function(Function),

  // HashMap(HashMap<Rc<str>, ()>)
}
