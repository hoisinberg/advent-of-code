use aoc::hoi;
use std::io;
use std::ops::RangeInclusive;
use std::vec::Vec;

fn count_digits(mut n: u64) -> u32 {
    let mut d = 0;
    while n > 0 {
        d += 1;
        n /= 10;
    }
    return d;
}

// Given n with decimal representation abc, return abcabc
fn duplicate(n: u64) -> u64 {
    multiduplicate(n, 1)
}

fn multiduplicate(n: u64, k: u32) -> u64 {
    let mut r = 0;
    for _ in 0..k {
        r += n * 10u64.pow(count_digits(n)) + n
    }
    r
}

fn effective_ceiling(ceiling: u64) -> u64 {
    let d = count_digits(ceiling);
    if d % 2 == 0 {
        ceiling
    } else {
        10u64.pow(d - 1) - 1
    }
}

fn halve_digits(n: u64) -> u64 {
    n / (10u64.pow(count_digits(n) / 2))
}

fn effective_floor(floor: u64) -> u64 {
    let d = count_digits(floor);
    if d % 2 == 0 { floor } else { 10u64.pow(d) }
}

fn sum_invalid_ids(range: &RangeInclusive<u64>) -> u64 {
    let seed_floor = halve_digits(effective_floor(*range.start()));
    let seed_ceiling = halve_digits(effective_ceiling(*range.end()));
    let invalid_ids: Vec<u64> = (seed_floor..=seed_ceiling)
        .map(&duplicate)
        .filter(|n| range.contains(n))
        .collect();
    println!(
        "actual: [{x}, {y}] eff: [{a}, {b}]",
        x = range.start(),
        y = range.end(),
        a = effective_floor(*range.start()),
        b = effective_ceiling(*range.end())
    );
    println!("seed: [{a}, {b}]", a = seed_floor, b = seed_ceiling);
    for i in 0..std::cmp::min(3, invalid_ids.len()) {
        println!("{s}", s = invalid_ids[i]);
    }
    invalid_ids.iter().sum()
}

fn parse_range(range: &str) -> io::Result<RangeInclusive<u64>> {
    let pieces = range
        .split('-')
        .map(|piece| {
            piece.parse::<u64>().map_err(|parse_error| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Unable to parse u32: {parse_error}"),
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if pieces.len() != 2 {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("More than 1 '-': {range}"),
        ))
    } else {
        Ok(pieces[0]..=(pieces[1]))
    }
}

fn part1(input: &Vec<String>) -> u64 {
    let ranges = input
        .iter()
        .flat_map(|line| line.split(','))
        .map(&parse_range)
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse input ranges");
    for range in &ranges {
        println!("[{a}, {b}]", a = range.start(), b = range.end());
    }
    ranges.iter().map(&sum_invalid_ids).sum()
}

fn main() {
    hoi::console::greet(2025, 2, 1);
    let input = hoi::file::read_lines("input/2025_day02.txt", &hoi::func::id_result)
        .expect("Couldn't read input.");
    println!("Part 1: {p1}", p1 = part1(&input));
}
