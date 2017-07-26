use chrono::{DateTime, FixedOffset, NaiveDateTime};
use chrono::offset::TimeZone;
use regex::Regex;
use serde_json;
use serde_json::Value;
use std::env;
use std::fmt;
use std::io;
use std::io::ErrorKind;
use std::process::Command;

use termion::color;

#[derive(Deserialize, Debug)]
pub struct LogLine {
  pub rev: u32,
  pub node: String,
  pub branch: String,
  pub phase: String,
  pub user: String,
  pub date: (i64, i32),
  pub desc: String,
  pub bookmarks: Vec<String>,
  pub tags: Vec<String>,
  pub parents: Vec<String>,
}

impl LogLine {
  fn name(&self) -> String {
    lazy_static! {
      static ref REGEX_USER: Regex = Regex::new(r"(.*)\s*<(.*)>").unwrap();
    }
    let cap = REGEX_USER.captures(&self.user).unwrap();
    cap[1].to_string()
  }

  fn time(&self) -> String {
    let ts = NaiveDateTime::from_timestamp(self.date.0, 0);
    let tz = TimeZone::from_offset(&FixedOffset::west(self.date.1));
    let dt = DateTime::<FixedOffset>::from_utc(ts, tz);
    format!("{}", dt.format("%Y-%m-%d %H:%M"))
  }
}

impl fmt::Display for LogLine {
  // This trait requires `fmt` with this exact signature.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,
           "{}{}{}",
           color::Fg(color::Red),
           &self.time(),
           color::Fg(color::Reset))?;
    write!(f,
           "  {}{}{}",
           color::Fg(color::Green),
           &self.name(),
           color::Fg(color::Reset))?;
    write!(f, "\t{}", &self.desc)?;
    write!(f, "{}", color::Fg(color::Reset))
  }
}

pub fn log() -> io::Result<Vec<LogLine>> {
  let output = Command::new("hg").arg("log").arg("-Tjson").output()?;
  let status = output.status;

  if status.success() {
    let logstr = String::from_utf8(output.stdout).unwrap();
    let log: Value = serde_json::from_str(&logstr).unwrap();
    Ok(serde_json::from_value(log).unwrap())
  } else {
    let msg = format!("No mercurial repository found in specified root: {}",
                      &env::current_dir().unwrap().display());
    Err(io::Error::new(ErrorKind::Other, msg))
  }
}

#[derive(Deserialize, Debug)]
pub struct StatusLine {
  path: String,
  status: char,
}

impl fmt::Display for StatusLine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.status {
      'M' => write!(f, "{}", color::Fg(color::LightBlue)),
      'A' => write!(f, "{}", color::Fg(color::Green)),
      'R' => write!(f, "{}", color::Fg(color::Red)),
      'C' => write!(f, "{}", color::Fg(color::Cyan)),
      '!' => write!(f, "{}", color::Fg(color::LightRed)),
      '?' => write!(f, "{}", color::Fg(color::LightMagenta)),
      'I' => write!(f, "{}", color::Fg(color::Yellow)),
      _ => unreachable!(),
    }?;

    write!(f, "{}\t{}", self.status, self.path)
  }
}

pub fn status() -> io::Result<Vec<StatusLine>> {
  let output = Command::new("hg").arg("status").arg("-Tjson").output()?;
  let status = output.status;

  if status.success() {
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Value = serde_json::from_str(&stdout).unwrap();
    Ok(serde_json::from_value(lines).unwrap())
  } else {
    let msg = format!("No mercurial repository found in specified root: {}",
                      &env::current_dir().unwrap().display());
    Err(io::Error::new(ErrorKind::Other, msg))
  }
}
