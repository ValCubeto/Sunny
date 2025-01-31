
#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::stdout_size;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::stdout_size;

#[cfg(not(any(windows, unix)))]
pub fn stdout_size() -> Option<(u16, u16)> {
  None
}
