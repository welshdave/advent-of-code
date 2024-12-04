use regex::Regex;

pub fn multiply_corrupt_numbers(input: &str) -> i32 {
    let re = Regex::new("mul\\([0-9]+,[0-9]+\\)").unwrap();
    re.find_iter(input).map(|x| multiply(x.as_str())).sum()
}

fn multiply(input: &str) -> i32 {
    let re = Regex::new("([0-9]+)").unwrap();
    let mut result = re.find_iter(input);
    result.next().unwrap().as_str().parse::<i32>().unwrap()
        * result.next().unwrap().as_str().parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_corrupt_numbers() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(multiply_corrupt_numbers(input), 161);
    }
}
