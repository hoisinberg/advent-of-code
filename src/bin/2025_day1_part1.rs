use aoc::hoi;
use std::io;

fn parse_rotation_sign(rotation: u8) -> io::Result<i32> {
    match rotation {
        b'R' => Ok(1),
        b'L' => Ok(-1),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unable to parse rotation type {rotation}"),
        )),
    }
}

fn parse_rotation(str: String) -> io::Result<i32> {
    let sign = parse_rotation_sign(str.as_bytes()[0])?;
    let distance = &str[1..].parse().map_err(|parse_error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unable to parse rotation distance: {parse_error}"),
        )
    })?;
    Ok(sign * distance)
}

fn rotate(dial_and_zero_count: (i32, u32), rotation: &i32) -> (i32, u32) {
    let (dial, zero_count) = dial_and_zero_count;
    let new_dial = (dial + rotation).rem_euclid(100);
    let zero_count_delta = if new_dial == 0 { 1 } else { 0 };
    (new_dial, zero_count + zero_count_delta)
}

fn main() {
    hoi::console::greet(2025, 1, 1);
    let rotations = hoi::file::read_lines("input/2025_day01.txt", &parse_rotation)
        .expect("Failed to read and parse rotations.");
    let (_, part1_zero_count) = rotations.iter().fold((50, 0), &rotate);
    println!("Part 1 answer: {part1_zero_count}");
}
