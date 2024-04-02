extern crate crossterm;
extern crate lazy_static;

#[allow(unused)]
mod colors;

use colors::*;
use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseEventKind},
  terminal::{self, Clear, ClearType}, ExecutableCommand,
};
use std::io::{self, Write};

/// "Print and flush", but it's the same as C's
macro_rules! printf {
  ($($arg: expr),*) => {{
  use ::std::io::Write;
  print!($($arg),*);
  ::std::io::stdout().flush().unwrap();
  }};
}

fn main() -> io::Result<()> {
  let prompt = "> ";

  println!("Welcome to REPL v0.1!");
  println!("Call {BOLD}help(){BOLD_END} for more information");
  println!();
  println!("{prompt}");
  println!();

  terminal::enable_raw_mode()?;
  let mut stdout = io::stdout();
  let stdin = io::stdin();
  let mut input = String::new();
  let mut in_multiline_mode = false;
  let mut cursor = 0;

  loop {
    match event::read()? {
      Event::Key(key_event) => match key_event.code {
        KeyCode::Char(ch) => {
          if ch == 'c' && key_event.modifiers.contains(KeyModifiers::CONTROL) {
            stdout.execute(DisableMouseCapture)?;
            break;
          }
          input.push(ch);
        }
        KeyCode::Enter => {
          terminal::disable_raw_mode()?;
          println!();
          println!("input: {input:?}");
          input = String::new();
        }
        KeyCode::Esc => {
          stdout.execute(DisableMouseCapture)?;
          println!();
          println!();
          println!("Goodbye!");
          break;
        },
        KeyCode::Backspace => {
          if !input.is_empty() {
            let _removed = input.remove(cursor);
          }
        }
        _ => {}
      },
      Event::Mouse(mouse_event) => match mouse_event.kind {
        MouseEventKind::Down(mouse_event) => {},
        MouseEventKind::Up(mouse_event) => {},
        MouseEventKind::ScrollLeft => {},
        MouseEventKind::ScrollRight => {},
        MouseEventKind::Drag(button) => {},
        MouseEventKind::Moved => {},
        MouseEventKind::ScrollDown => {},
        MouseEventKind::ScrollUp => {}
      },
      Event::Paste(text) => {},
      Event::Resize(x, y) => {},
      _ => {}
    }
  }
  Ok(())
}
