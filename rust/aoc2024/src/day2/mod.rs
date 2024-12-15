use std::fs;

pub fn part1(file_name: &str) -> u32 {
    let current_file = file!();
    let input1_path = std::path::Path::new(current_file)
        .parent()
        .unwrap()
        .join(file_name);
    let content = fs::read_to_string(input1_path).unwrap();

    let input = parse_input(content.as_str());
    let mut count_reports: u32 = 0;
    for line in input {
        let increase_or_decrease = line[1] - line[0];
        for (i, val) in line[1..].iter().enumerate() {
            let diff = val - line[i];
            if diff.abs() > 3 || (increase_or_decrease * diff) <= 0 {
                break;
            }
            if i + 2 == line.len() {
                count_reports += 1;
            }
        }
    }
    count_reports
}

pub fn part2(file_name: &str) -> u32 {
    let current_file = file!();
    let input1_path = std::path::Path::new(current_file)
        .parent()
        .unwrap()
        .join(file_name);
    let content = fs::read_to_string(input1_path).unwrap();

    let input = parse_input(content.as_str());
    let mut count_reports: u32 = 0;
    for line in input {
        if is_safe(line, 1) {
            count_reports += 1;
        }
    }
    count_reports
}

fn is_safe(line: Vec<i32>, tolerance: i32) -> bool {
    let mut increase_or_decrease = 0;
    for (i, val) in line[1..].iter().enumerate() {
        let diff = val - line[i];
        if diff != 0 {
            increase_or_decrease += diff / diff.abs();
        }
    }
    for (i, val) in line[1..].iter().enumerate() {
        let diff = val - line[i];
        if diff.abs() > 3 || (increase_or_decrease * diff) <= 0 {
            if tolerance <= 0 {
                return false;
            }
            let mut new_vec1 = line.clone();
            new_vec1.remove(i + 1);
            let mut new_vec2 = line.clone();
            new_vec2.remove(i);
            return is_safe(new_vec1, tolerance - 1) || is_safe(new_vec2, tolerance - 1);
        }
    }
    true
}

pub fn parse_input(input_txt: &str) -> Vec<Vec<i32>> {
    input_txt
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part2("input2.txt"), 418);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input1.txt"), 359);
    }

    #[test]
    fn test_simple_input2() {
        assert_eq!(part2("simple_input.txt"), 4);
    }

    #[test]
    fn test_simple_input1() {
        assert_eq!(part1("simple_input.txt"), 2);
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            ),
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9]
            ]
        );
    }
}
