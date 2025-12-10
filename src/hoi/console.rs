use std::io::{self};

pub fn greet(year: u32, day: u32, part: u32) {
    println!("Advent of Code {year}, day {day}, part {part}");
}

pub fn get_arg(index: usize) -> io::Result<String> {
  match std::env::args().nth(index) {
    None => Err(io::Error::new(
      io::ErrorKind::NotFound,
      format!("Missing argument {index}"),
    )),
    Some(arg) => Ok(arg),
  }
}
