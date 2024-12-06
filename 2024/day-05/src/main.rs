use std::fs;
use day_05::part1::correctly_ordered_middle_pages;
use day_05::part2::incorrectly_ordered_middle_pages;

fn main() {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");

    let input = fs::read_to_string(base_path.join("day-05/input/data.txt")).unwrap();

    println!("Part 1 result: {:?}", correctly_ordered_middle_pages(input.as_str()));

    println!("Part 2 result: {:?}", incorrectly_ordered_middle_pages(input.as_str()));
}