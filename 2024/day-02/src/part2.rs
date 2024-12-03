use std::str::FromStr;

pub fn check_safety_with_dampener(input: &str) -> i16 {
    let mut count = 0;

    for line in input.lines() {
        let items: Vec<i16> = line
            .split_whitespace()
            .filter_map(|num| i16::from_str(num).ok())
            .collect();
        if check_safe(items.clone()) {
            count += 1;
        } else {
            let mut found = false;
            for i in 0..items.len() {
                if found {
                    continue;
                }
                let mut modified = items.to_vec();
                modified.remove(i);

                if check_safe(modified.to_vec()) {
                    count += 1;
                    found = true;
                    continue;
                }
            }
        }
    }

    count
}

fn check_safe(items: Vec<i16>) -> bool {
    if items
        .windows(2)
        .all(|w| (w[0] < w[1]) && (w[1] - w[0]) <= 3)
    {
        return true;
    }

    if items
        .windows(2)
        .all(|w| (w[0] > w[1]) && (w[0] - w[1]) <= 3)
    {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety_with_dampener() {
        let input = "\
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";

        assert_eq!(check_safety_with_dampener(input), 4);
    }
}
