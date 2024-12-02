pub fn calculate_distance(input: &str) -> i32 {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<i32>().unwrap());
        right.push(items.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let result: i32 = std::iter::zip(left, right)
        .map(|(a, b)| (a - b).abs())
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let input = "\
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3";
        assert_eq!(calculate_distance(input), 11);
    }
}
