use std::io;
use std::process::Command;

pub fn log() -> io::Result<Vec<u8>> {
  return match Command::new("hg").arg("log").output() {
    Ok(output) => Ok(output.stdout),
    Err(e) => Err(e),
  };
}
