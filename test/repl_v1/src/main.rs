extern crate crossterm;
extern crate lazy_static;

#[allow(unused)]
mod colors;

use colors::*;
use crossterm::{
  cursor::MoveTo, event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind}, terminal::{self, Clear, ClearType}, ExecutableCommand
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
  terminal::enable_raw_mode()?;

  println!("Welcome to REPL v0.1!");
  println!("Call {BOLD}help(){BOLD_END} for more information");
  println!();
  println!("{prompt}");

  let mut stdout = io::stdout();
  let stdin = io::stdin();
  let mut input = String::new();
  let mut in_multiline_mode = false;
  let mut cursor = 0;

  stdout.execute(EnableMouseCapture)?;
  loop {
    #[allow(clippy::single_match)]
    match event::read()? {
      // NOTE: the event is triggered on both pressing and releasing,
      // so if not handled correctly it gets keys twice.
      Event::Key(key_event) if key_event.kind != KeyEventKind::Release => match key_event.code {
        KeyCode::Char(ch) => {
          if key_event.modifiers.contains(KeyModifiers::CONTROL) {
            match ch {
              'c' => {
                terminal::disable_raw_mode()?;
                println!();
                print!("Goodbye!");
                break;
              },
              // 'm' => {
              //   in_multiline_mode = !in_multiline_mode;
              // }
              _ => {
                // stdout.execute(DisableMouseCapture)?;
                // println!();
                // println!("Pressed Ctrl+{ch:?}");
                // stdout.execute(EnableMouseCapture)?;
              }
            }
          }
          cursor += 1;
          input.push(ch);
        }
        KeyCode::Enter => {
          terminal::disable_raw_mode()?;
          println!();
          println!("input: {input:?}");
          println!("{prompt}");
          cursor = 0;
          input = String::new();
          terminal::enable_raw_mode()?;
        }
        KeyCode::Backspace => {
          if !input.is_empty() {
            let _removed = input.remove(cursor - 1);
            cursor -= 1;
          }
        }
        KeyCode::Delete => {
          //
        }
        KeyCode::Home => {
          cursor = 0;
          // stdout.execute(MoveTo(?, ?));
        }
        KeyCode::End => {
          cursor = input.len() - 1;
          // move real cursor
          // stdout.execute(MoveTo(?, ?));
        }
        _ => {}
      },
      // Event::Mouse(mouse_event) => match mouse_event.kind {
      //   MouseEventKind::Down(mouse_event) => {},
      //   MouseEventKind::Up(mouse_event) => {},
      //   MouseEventKind::ScrollLeft => {},
      //   MouseEventKind::ScrollRight => {},
      //   MouseEventKind::Drag(button) => {},
      //   MouseEventKind::Moved => {},
      //   MouseEventKind::ScrollDown => {},
      //   MouseEventKind::ScrollUp => {}
      // },
      // Event::Paste(text) => {},
      // Event::Resize(x, y) => {},
      _ => {}
    }
  }
  Ok(())
}
