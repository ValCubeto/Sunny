use std::io::stdout;
use std::os::unix::io::{ AsFd, BorrowedFd, RawFd };
use rustix::termios::{ isatty as is_a_tty, tcgetwinsize as get_win_size };

pub fn stdout_size() -> Option<(u16, u16)> {
  let stream = stdout();
  if !is_a_tty(&stream) {
    return None;
  }
  let win_size = get_win_size(&stream).ok()?;
  let (cols, rows) = (win_size.ws_col, win_size.ws_row);
  if cols > 0 && rows > 0 {
    Some((cols, rows))
  } else {
    None
  }
}
