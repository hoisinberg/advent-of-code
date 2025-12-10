use aoc::hoi;

fn main() {
    hoi::console::greet(2025, 1, 1);

    let lines = hoi::file::read_lines("input/2025_day01.txt", &hoi::func::id_result)
        .expect("Couldn't read file.");
    for line in lines {
        println!("{line}");
    }
}
