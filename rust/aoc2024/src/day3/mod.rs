use regex::Regex;

pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    for captured in re.captures_iter(input) {
        let left = captured.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let right = captured.get(2).unwrap().as_str().parse::<i32>().unwrap();
        sum += left * right;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1_simple() {
        assert_eq!(
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        )
    }

    #[test]
    fn test_part1_solution() {
        let current_file = file!();
        let input1_path = std::path::Path::new(current_file)
            .parent()
            .unwrap()
            .join("input1.txt");
        let content = fs::read_to_string(input1_path).unwrap();

        assert_eq!(part1(&*content), 174960292);
    }
}
