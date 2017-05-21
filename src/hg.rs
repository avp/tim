use serde_json;
use serde_json::Value;
use std::io;
use std::io::ErrorKind;
use std::process::Command;

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
