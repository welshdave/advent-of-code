use std::collections::HashMap;

pub fn calculate_similarity(input: &str) -> i32 {
    let mut left = vec![];
    let mut right: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<i32>().unwrap());
        right
            .entry(items.next().unwrap().parse::<i32>().unwrap())
            .and_modify(|f| *f += 1)
            .or_insert(1);
    }

    left.iter()
        .map(|num| num * right.get(num).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similarity() {
        let input = "\
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3";
        assert_eq!(31, calculate_similarity(input));
    }
}
