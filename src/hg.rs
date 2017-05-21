use serde_json;
use serde_json::Value;
use std::io;
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

impl ToString for LogLine {
  fn to_string(&self) -> String {
    let mut result = String::new();
    result += &self.user;
    result += &self.desc;
    result
  }
}

pub fn log() -> io::Result<Vec<LogLine>> {
  let output = Command::new("hg").arg("log").arg("-Tjson").output()?;
  let log: Value = serde_json::from_str(&String::from_utf8(output.stdout).unwrap())
    .expect("Invalid hg log output");
  Ok(serde_json::from_value(log).unwrap())
}
