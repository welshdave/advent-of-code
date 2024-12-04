use crate::part1::multiply_corrupt_numbers;

pub fn multiply_conditional_corrupt_numbers(input: &str) -> i32 {
    let mut segments = input.split("don't");
    let mut result = multiply_corrupt_numbers(segments.next().unwrap_or(""));

    for segment in segments {
        result += multiply_corrupt_numbers(&segment[segment.find("do").unwrap_or(segment.len())..])
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_corrupt_numbers() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(multiply_conditional_corrupt_numbers(input), 48);
    }
}
