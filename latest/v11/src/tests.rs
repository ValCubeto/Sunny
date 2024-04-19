#[test]
fn relative_ptrs() {
  let string = "Hello, world!";
  // const std::mem::VERY_START: &()
  let very_start = string.as_ptr() as usize;
  let my_relative_ptr: u32 = 7;
  let letter_w = unsafe {
    *((very_start + my_relative_ptr as usize) as *const u8) as char
  };
  assert_eq!(letter_w, 'w');
  assert_ne!(letter_w, 'd');
}
