use day_01::part1::calculate_distance;
use day_01::part2::calculate_similarity;
use std::fs;

fn main() {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");

    let input = fs::read_to_string(base_path.join("day-01/input/list.txt")).unwrap();

    println!("Part 1 result: {:?}", calculate_distance(input.as_str()));

    println!("Part 2 result: {:?}", calculate_similarity(input.as_str()));
}
