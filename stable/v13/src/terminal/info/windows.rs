use std::os::windows::io::{ AsHandle, AsRawHandle, BorrowedHandle, RawHandle };
use windows_sys::Win32::Foundation::{ HANDLE as Handle, INVALID_HANDLE_VALUE };
use windows_sys::Win32::System::Console::{
  GetStdHandle as get_std_handle,
  STD_OUTPUT_HANDLE,
  COORD as Coords,
  SMALL_RECT as SmallRect,
  CONSOLE_SCREEN_BUFFER_INFO as ConsoleScreenBufferInfo,
  GetConsoleScreenBufferInfo as get_console_screen_buffer_info,
};

fn stdout_handle() -> Option<Handle> {
  let handle = unsafe {
    BorrowedHandle::borrow_raw(get_std_handle(STD_OUTPUT_HANDLE))
  };
  let handle = handle.as_raw_handle() as Handle;
  if handle == INVALID_HANDLE_VALUE {
    return None;
  }
  Some(handle)
}

/// (width, height)
fn size_of(handle: Handle) -> Option<(u16, u16)> {
  let coords = Coords { X: 0, Y: 0 };
  // Create an empty struct to receive the info from the function
  let mut info = ConsoleScreenBufferInfo {
    dwSize: coords,
    dwCursorPosition: coords,
    wAttributes: 0,
    srWindow: SmallRect {
      Left: 0,
      Top: 0,
      Right: 0,
      Bottom: 0
    },
    dwMaximumWindowSize: coords
  };
  if unsafe { get_console_screen_buffer_info(handle, &mut info) } == 0 {
    return None;
  }
  // Avoid negative values
  let rows = (info.srWindow.Bottom - info.srWindow.Top).max(0);
  let cols = (info.srWindow.Right - info.srWindow.Left).max(0);
  Some((rows as u16, cols as u16))
}

pub fn stdout_size() -> Option<(u16, u16)> {
  stdout_handle().and_then(size_of)
}
