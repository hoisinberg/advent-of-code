use aoc::hoi;
use std::io;

fn main() {
    hoi::console::greet(2025, 2, 1);
    let lines = hoi::file::read_lines("input/2025_day02.txt", &hoi::func::id_result)
        .expect("Couldn't read input.");
    for line in lines {
        println!("{line}");
    }
}
