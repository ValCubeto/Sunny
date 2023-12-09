pub struct Value {
  id: usize,
  ptr: usize,
  len: usize,
}

// values = Vec<usize> -> elem as *const u8

impl Value {
  #[inline]
  pub fn new(id: usize, ptr: usize, len: usize) -> Self {
    Value { id, ptr, len }
  }
}