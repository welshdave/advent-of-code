use std::fs;
use day_04::part1::find_xmas;

fn main() {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");

    let input = fs::read_to_string(base_path.join("day-04/input/data.txt")).unwrap();

    println!(
        "Part 1 result: {:?}",
        find_xmas(input.as_str())
    );

}
