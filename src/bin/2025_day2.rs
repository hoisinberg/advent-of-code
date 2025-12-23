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
    multiduplicate(n, 2)
}

fn multiduplicate(n: u64, k: u32) -> u64 {
    let mut r = 0;
    let shift_factor = 10u64.pow(count_digits(n));
    for _ in 0..k {
        r = (r * shift_factor) + n
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

fn is_invalid_id(id: u64) -> bool {
    let digits = count_digits(id);
    for n in 1..=(digits / 2) {
        if digits % n != 0 {
            continue;
        }
        let q = digits / n;
        let last_n_digits = id % (10u64.pow(n));
        if multiduplicate(last_n_digits, q) == id {
            return true;
        }
    }
    return false;
}

fn sum_invalid_ids(range: &RangeInclusive<u64>) -> u64 {
    let seed_floor = halve_digits(effective_floor(*range.start()));
    let seed_ceiling = halve_digits(effective_ceiling(*range.end()));
    let invalid_ids: Vec<u64> = (seed_floor..=seed_ceiling)
        .map(&duplicate)
        .filter(|n| range.contains(n))
        .collect();
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
    ranges.iter().map(&sum_invalid_ids).sum()
}

fn part2(input: &Vec<String>) -> u64 {
    let ranges = input
        .iter()
        .flat_map(|line| line.split(','))
        .map(&parse_range)
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse input ranges");
    ranges
        .into_iter()
        .flat_map(|range| range.into_iter())
        .filter(|id| is_invalid_id(*id))
        .sum()
}

fn main() {
    hoi::console::greet(2025, 2, 1);
    let input = hoi::file::read_lines("input/2025_day02.txt", &hoi::func::id_result)
        .expect("Couldn't read input.");
    println!("Part 1: {p1}", p1 = part1(&input));

    println!("Part 2: {p2}", p2 = part2(&input));
}
