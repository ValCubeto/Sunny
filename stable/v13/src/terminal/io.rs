
/// (width, height)
#[cfg(windows)]
pub fn stdout_size() -> Option<(u16, u16)> {
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

  let handle = unsafe {
    BorrowedHandle::borrow_raw(get_std_handle(STD_OUTPUT_HANDLE))
  };
  let handle = handle.as_raw_handle() as Handle;
  if handle == INVALID_HANDLE_VALUE {
    return None;
  }
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
  let rows = (info.srWindow.Bottom - info.srWindow.Top + 1).max(0);
  let cols = (info.srWindow.Right - info.srWindow.Left + 1).max(0);
  Some((rows as u16, cols as u16))
}

#[cfg(unix)]
pub fn stdout_size() -> Option<(u16, u16)> {
  use std::io::stdout;
  use std::os::unix::io::{ AsFd, BorrowedFd, RawFd };
  use rustix::termios::{ isatty as is_a_tty, tcgetwinsize as get_win_size };

  let fd = stdout();
  if !is_a_tty(&fd) {
    return None;
  }
  let win_size = get_win_size(&fd).ok()?;
  let (rows, cols) = (win_size.ws_row, win_size.ws_col);
  if rows > 0 && cols > 0 {
    Some((rows as u16, cols as u16))
  } else {
    None
  }
}

#[cfg(not(any(windows, unix)))]
pub fn stdout_size() -> Option<(u16, u16)> {
  None
}
