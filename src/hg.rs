use regex::Regex;
use serde_json;
use serde_json::Value;
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
  pub date: Vec<u64>,
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
    format!("{}", &cap[1])
  }
}

impl fmt::Display for LogLine {
  // This trait requires `fmt` with this exact signature.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.rev)?;
    write!(f,
           "\t{}{}{}",
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
    Err(io::Error::new(ErrorKind::Other, "No mercurial repository found"))
  }
}
