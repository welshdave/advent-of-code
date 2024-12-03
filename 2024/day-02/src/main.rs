use day_02::part1::check_safety;
use day_02::part2::check_safety_with_dampener;
use std::fs;

fn main() {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");

    let input = fs::read_to_string(base_path.join("day-02/input/list.txt")).unwrap();

    println!("Part 1 result: {:?}", check_safety(input.as_str()));

    println!(
        "Part 2 result: {:?}",
        check_safety_with_dampener(input.trim())
    );
}
