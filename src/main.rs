extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate termion;

use std::env;
use std::io::{Write, stdin, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod hg;

fn main() {
  let dir = env::args().nth(1).unwrap_or(String::from("."));
  env::set_current_dir(dir).expect("Invalid directory was specified");

  // Get the standard input stream.
  let stdin = stdin();
  // Get the standard output stream and go to raw mode.
  let mut stdout = stdout().into_raw_mode().unwrap();

  let log = match hg::log() {
    Ok(l) => {
      let mut result = String::new();
      for c in l {
        result += &format!("\r{} {}\n", c.user, c.desc);
      }
      result
    }
    Err(_) => String::from("Failed to fetch log"),
  };

  write!(stdout,
         "{}{}{}{}",
         // Clear the screen.
         termion::clear::All,
         // Goto (1,1).
         termion::cursor::Goto(1, 1),
         // Output the log.
         log,
         // Hide the cursor.
         termion::cursor::Hide)
      .unwrap();
  // Flush stdout (i.e. make the output appear).
  stdout.flush().unwrap();

  for c in stdin.keys() {
    // Print the key we type...
    match c.unwrap() {
      // Exit.
      Key::Char('q') => break,
      Key::Char(c) => println!("{}", c),
      Key::Alt(c) => println!("Alt-{}", c),
      Key::Ctrl(c) => println!("Ctrl-{}", c),
      Key::Left => println!("<left>"),
      Key::Right => println!("<right>"),
      Key::Up => println!("<up>"),
      Key::Down => println!("<down>"),
      _ => println!("Other"),
    }

    // Flush again.
    stdout.flush().unwrap();
  }

  // Clear the current line.
  write!(stdout,
         "{}{}",
         termion::cursor::Goto(1, 1),
         termion::clear::All)
    .unwrap();

  // Show the cursor again before we exit.
  write!(stdout, "{}", termion::cursor::Show).unwrap();
}
