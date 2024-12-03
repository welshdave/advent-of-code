use std::str::FromStr;

pub fn check_safety(input: &str) -> i16 {
    let mut count: i16 = 0;

    for line in input.lines() {
        let items: Vec<i16> = line
            .split_whitespace()
            .filter_map(|num| i16::from_str(num).ok())
            .collect();
        if items
            .windows(2)
            .all(|w| (w[0] < w[1]) && (w[1] - w[0]) <= 3)
        {
            count += 1;
            continue;
        }

        if items
            .windows(2)
            .all(|w| (w[0] > w[1]) && (w[0] - w[1]) <= 3)
        {
            count += 1;
            continue;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety() {
        let input = "\
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";

        assert_eq!(check_safety(input), 2);
    }
}
