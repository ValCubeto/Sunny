#[test]
fn relative_ptrs() {
  let string = "Hello, world!";
  // const std::mem::VERY_START: &()
  let very_start = string.as_ptr() as usize;

  let deref = |ptr: u32| -> char {
    unsafe {
      *((very_start + (ptr as usize)) as *const u8) as char
    }
  };

  let my_relative_ptr: u32 = 7;
  let letter_w = deref(my_relative_ptr);
  assert_eq!(letter_w, 'w');
  assert_ne!(letter_w, 'd');
}
