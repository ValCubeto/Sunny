extern crate crossterm;
extern crate lazy_static;

#[allow(unused)]
mod colors;

use colors::*;
use crossterm::{
  cursor::{ MoveLeft, MoveRight, MoveUp },
  event::{
    read as read_event,
    DisableMouseCapture,
    EnableMouseCapture,
    Event,
    KeyCode,
    KeyEventKind,
    KeyModifiers
  },
  terminal,
  ExecutableCommand as _
};
use std::io;

fn main() -> io::Result<()> {
  let prompt = "> ";
  let mut stdout = io::stdout();

  terminal::enable_raw_mode()?;
  println!("Welcome to REPL v0.1!");
  println!("Call {BOLD}help(){BOLD_END} for more information");
  println!();
  println!("{prompt}");
  stdout.execute(MoveUp(1))?;
  stdout.execute(MoveRight(prompt.len() as u16))?;
  stdout.execute(EnableMouseCapture)?;

  // let stdin = io::stdin();
  let mut input = String::new();
  let mut in_multiline_mode = false;
  let mut cursor = 0;

  loop {
    #[allow(clippy::single_match)]
    match read_event()? {
      // NOTE: the event is triggered on both pressing and releasing,
      // so if not handled correctly it gets keys twice.
      Event::Key(key_event) if key_event.kind != KeyEventKind::Release => match key_event.code {
        KeyCode::Char(ch) => {
          if key_event.modifiers.contains(KeyModifiers::CONTROL) {
            match ch {
              'c' => {
                terminal::disable_raw_mode()?;
                stdout.execute(DisableMouseCapture)?;
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
            continue;
          }
          cursor += 1;
          stdout.execute(MoveRight(1))?;
          input.insert(cursor - 1, ch);
        }
        KeyCode::Enter => {
          terminal::disable_raw_mode()?;
          println!();
          println!("input: {input:?}");
          println!("{prompt}");
          cursor = 0;
          input = String::new();
          stdout.execute(MoveUp(1))?;
          stdout.execute(MoveRight(prompt.len() as u16))?;
          terminal::enable_raw_mode()?;
        }
        KeyCode::Backspace => {
          if !input.is_empty() {
            let _removed = input.remove(cursor - 1);
            cursor -= 1;
            stdout.execute(MoveLeft(1))?;
          }
        }
        KeyCode::Delete => {
          if !input.is_empty() && cursor < input.len() {
            let _removed = input.remove(cursor);
            cursor -= 1;
          }
        }
        KeyCode::Home => {
          if cursor != 0 {
            stdout.execute(MoveLeft(cursor as u16))?;
            cursor = 0;
          }
        }
        KeyCode::End => {
          if cursor != input.len() {
            stdout.execute(MoveRight((input.len() - cursor) as u16))?;
            cursor = input.len();
          }
        }
        KeyCode::Left => {
          if cursor > 0 {
            terminal::disable_raw_mode()?;
            cursor -= 1;
            stdout.execute(MoveLeft(1))?;
            terminal::enable_raw_mode()?;
          }
        }
        KeyCode::Right => {
          if cursor < input.len() {
            cursor += 1;
            stdout.execute(MoveRight(1))?;
          }
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
