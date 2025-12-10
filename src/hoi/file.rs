use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

pub fn read_lines(path: &str) -> io::Result<Vec<String>> {
  let file = File::open(Path::new(path))?;
  let mut lines: Vec<String> = Vec::new();
  for line in io::BufReader::new(file).lines() {
    lines.push(line?)
  }
  Ok(lines)
}
