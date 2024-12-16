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

pub fn part2(input: &str) -> i32 {
    let re_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let dos = Regex::new(r"(do\(\))")
        .unwrap()
        .captures_iter(input)
        .map(|val| val.get(1).unwrap().start())
        .collect();

    let donts = Regex::new(r"(don't\(\))")
        .unwrap()
        .captures_iter(input)
        .map(|val| val.get(1).unwrap().start())
        .collect();

    let mut sum = 0;
    for captured in re_mul.captures_iter(input) {
        let left = captured.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let right = captured.get(2).unwrap().as_str().parse::<i32>().unwrap();

        if can_mul(&dos, &donts, captured.get(1).unwrap().start()) {
            sum += left * right;
        }
    }
    sum
}

fn can_mul(dos: &Vec<usize>, donts: &Vec<usize>, mul_pos: usize) -> bool {
    let mut closest_do = 0;

    for pos_do in dos {
        if pos_do < &mul_pos {
            closest_do = *pos_do;
        } else {
            break;
        }
    }

    let mut closes_dont: Option<usize> = None;
    for pos_dont in donts {
        if pos_dont < &mul_pos {
            closes_dont = Some(*pos_dont);
        } else {
            break;
        }
    }
    if closes_dont.is_none() {
        return true;
    }
    closest_do > closes_dont.unwrap()
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

    #[test]
    fn test_part2_solution() {
        let current_file = file!();
        let input1_path = std::path::Path::new(current_file)
            .parent()
            .unwrap()
            .join("input1.txt");
        let content = fs::read_to_string(input1_path).unwrap();

        assert_eq!(part2(&*content), 56275602);
    }

    #[test]
    fn test_part2_simple() {
        assert_eq!(
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        )
    }
}
