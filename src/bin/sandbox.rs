use std::convert::identity;
use aoc::hoi;

fn main() {
    let path = hoi::console::get_arg(1).expect("No path specified");
    println!("Path: {path}");

    let lines = hoi::file::read_lines(&path, &identity).expect("Couldn't read file.");
    for line in lines {
        println!("{line}");
    }
}
