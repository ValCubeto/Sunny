use std::io::stdout;
use std::os::unix::io::{ AsFd, BorrowedFd, RawFd };
use rustix::termios::{ isatty as is_a_tty, tcgetwinsize as get_win_size };

fn stdout() -> Option<RawFd> {
  let fd = stdout();
  if !is_a_tty(&fd) {
    return None;
  }
  Some(fd)
}

fn size_of(stream: RawFd) -> Option<(u16, u16)> {
  let win_size = get_win_size(&fd).ok()?;
  let (rows, cols) = (win_size.ws_row, win_size.ws_col);
  if rows > 0 && cols > 0 {
    Some((rows as u16, cols as u16))
  } else {
    None
  }
}

pub fn stdout_size() -> Option<(u16, u16)> {
  stdout().and_then(size_of)
}
