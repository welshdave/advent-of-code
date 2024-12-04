use day_03::part1::multiply_corrupt_numbers;
use day_03::part2::multiply_conditional_corrupt_numbers;
use std::fs;

fn main() {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");

    let input = fs::read_to_string(base_path.join("day-03/input/data.txt")).unwrap();

    println!(
        "Part 1 result: {:?}",
        multiply_corrupt_numbers(input.as_str())
    );

    println!(
        "Part 2 result: {:?}",
        multiply_conditional_corrupt_numbers(input.as_str())
    );
}
