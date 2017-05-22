extern crate chrono;

#[macro_use]
extern crate lazy_static;

extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate termion;

extern crate regex;

use std::env;
use std::io::{Write, stdin, stdout};
use std::process;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod hg;

fn main() {
  let dir = env::args().nth(1).unwrap_or(String::from("."));
  env::set_current_dir(&dir).expect("Invalid directory specified");

  // Get the standard input stream.
  let stdin = stdin();

  let log = match hg::log() {
    Ok(lines) => {
      let mut result = String::new();
      for line in lines {
        result += &format!("\r{}\n", line);
      }
      result
    }
    Err(_) => {
      println!("No mercurial repository found in specified root: {}",
               &env::current_dir().unwrap().display());
      process::exit(255);
    }
  };

  // Get the standard output stream and go to raw mode.
  let mut stdout = stdout().into_raw_mode().unwrap();

  write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide).unwrap();

  write!(stdout, "{}{}", termion::cursor::Goto(1, 1), log).unwrap();
  stdout.flush().unwrap();

  for c in stdin.keys() {
    // Print the key we type...
    match c.unwrap() {
      // Exit.
      Key::Char('q') => break,
      _ => {}
    }
  }

  write!(stdout,
         "{}{}",
         termion::cursor::Goto(1, 1),
         termion::clear::All)
      .unwrap();

  // Show the cursor again before we exit.
  write!(stdout, "{}", termion::cursor::Show).unwrap();

  stdout.flush().unwrap();
}
