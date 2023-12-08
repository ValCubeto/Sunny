pub struct Value {
  ptr: usize,
  len: usize,
  id: usize,
}

impl Value {
  #[inline]
  pub fn new(ptr: usize, len: usize, id: usize) -> Self {
    Value { ptr, len, id }
  }
}