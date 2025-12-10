use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

pub fn read_lines<T, P>(path: &str, parser: &P) -> io::Result<Vec<T>>
where
    P: Fn(String) -> T,
{
    let file = File::open(Path::new(path))?;
    let mut lines: Vec<T> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        lines.push(parser(line?));
    }
    Ok(lines)
}
