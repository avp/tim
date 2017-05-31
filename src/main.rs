extern crate chrono;

extern crate clap;

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

const TIM_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const TIM_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn get_args<'a>() -> clap::ArgMatches<'a> {
  clap::App::new("tim")
    .version(TIM_VERSION)
    .author(TIM_AUTHORS)
    .about("Text user interface for Mercurial")
    .arg(clap::Arg::with_name("COMMAND")
           .index(1)
           .default_value("log")
           .possible_values(&["log", "status"])
           .help("The command to run"))
    .arg(clap::Arg::with_name("repo")
           .short("r")
           .long("repo")
           .value_name("DIR")
           .help("The directory containing the Mercurial repository"))
    .get_matches()
}

fn main() {
  let args = get_args();
  let dir = args.value_of("repo").unwrap_or(".");
  env::set_current_dir(dir).expect("Invalid directory specified");

  // Get the standard input stream.
  let stdin = stdin();

  let display_str: String = match args.value_of("COMMAND").unwrap() {
    "log" => {
      match hg::log() {
        Ok(lines) => {
          let mut result = String::new();
          for line in lines {
            result += &format!("\r{}\n", line);
          }
          result
        }
        Err(e) => {
          println!("{}", e);
          process::exit(255);
        }
      }
    }
    "status" => {
      match hg::status() {
        Ok(lines) => {
          let mut result = String::new();
          for line in lines {
            result += &format!("\r{}\n", line);
          }
          result
        }
        Err(e) => {
          println!("{}", e);
          process::exit(255);
        }
      }
    }
    _ => unreachable!(),
  };

  // Get the standard output stream and go to raw mode.
  let mut stdout = stdout().into_raw_mode().unwrap();

  write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide).unwrap();

  write!(stdout, "{}{}", termion::cursor::Goto(1, 1), display_str).unwrap();
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
