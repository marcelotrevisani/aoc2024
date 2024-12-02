use std::fs;
use regex::Regex;


pub fn part1() -> u32{
    let current_file = file!();
    let input1_path = std::path::Path::new(current_file).parent().unwrap().join("input1.txt");
    let content = fs::read_to_string(input1_path).unwrap();

    let (mut left, mut right) = parse_input_day1(content.as_str());
    day1_part1(&mut left, &mut right)
}

pub fn parse_input_day1(input_txt: &str) -> (Vec<u32>, Vec<u32>) {
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let mut left_list: Vec<u32> = vec![];
    let mut right_list: Vec<u32> = vec![];
    for captured in re.captures_iter(input_txt) {
        left_list.push(captured.get(1).unwrap().as_str().parse::<u32>().unwrap());
        right_list.push(captured.get(2).unwrap().as_str().parse::<u32>().unwrap());
    }
    (left_list, right_list)
}

pub fn day1_part1(left_list: &mut Vec<u32>, right_list: &mut Vec<u32>) -> u32 {
    left_list.sort();
    right_list.sort();

    left_list.iter()
        .zip(right_list.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_day1() {
        assert_eq!(parse_input_day1(
            "3   4
4   3
2   5
1   3
3   9
3   3"), (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]));
    }

    #[test]
    fn test_day1_part1_simple(){
        assert_eq!(day1_part1(
            &mut vec![3, 4, 2, 1, 3, 3],
            &mut vec![4, 3, 5, 3, 9, 3]),
                   11
        );
    }

    #[test]
    fn test_day1_part1_solution(){
        assert_eq!(part1(), 2031679);
    }
}