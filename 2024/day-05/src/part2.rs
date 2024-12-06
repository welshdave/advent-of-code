use std::cmp::Ordering;
use std::collections::HashSet;

pub fn incorrectly_ordered_middle_pages(input: &str) -> i32 {
    let mut rules: HashSet<(usize, usize)> = HashSet::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();
    let mut result = 0;
    for line in input.trim().lines() {
        if line.contains('|') {
            let (before, after) = line
                .trim()
                .split_once("|")
                .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
                .unwrap();

            rules.insert((before, after));
        }
        if line.contains(',') {
            updates.push(
                line.trim()
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            );
        }
    }

    let is_valid = |pages: &[usize]| pages.windows(2).all(|pair| rules.contains(&(pair[0], pair[1])));

    for mut pages in updates {
        if !is_valid(&pages) {
            pages.sort_by(|&before, &after| {
                if rules.contains(&(before, after)) {
                    Ordering::Less
                } else if rules.contains(&(after, before)) {
                    Ordering::Greater
                } else {
                    panic!("error")
                }
            });
            
            result += pages[pages.len() / 2] as i32;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incorrectly_ordered_middle_pages() {
        let input = "\
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47";
        let result = incorrectly_ordered_middle_pages(input);
        assert_eq!(result, 123);
    }
}
