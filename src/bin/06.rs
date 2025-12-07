use std::{collections::VecDeque, vec};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut values = vec![];
    let lines: Vec<&str> = input.trim().lines().collect();
    let mut answer = 0;
    for (line_num, line) in lines.iter().enumerate() {
        for (value_index, value) in line.split_ascii_whitespace().enumerate() {
            if line_num < lines.len() - 1 {
                if line_num == 0 {
                    values.push(vec![value.parse::<u64>().unwrap()]);
                } else {
                    values[value_index].push(value.parse::<u64>().unwrap());
                }
            } else {
                answer += use_operator_on(value, &values[value_index]);
            }
        }
    }
    return Some(answer);
}

pub fn use_operator_on(operator: &str, array: &Vec<u64>) -> u64 {
    match operator {
        "+" => array.iter().fold(0, |acc, x| acc + x),
        "*" => array.iter().fold(1, |acc, x| acc * x),
        _ => {
            panic!("unknown value")
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.trim().lines().collect();
    let mut buckets = vec![vec![]; lines[0].len()];
    let mut answer = 0;
    for (line_num, line) in lines.iter().enumerate() {
        for (value_index, value) in line.chars().enumerate() {
            if line_num < lines.len() - 1 {
                buckets[value_index].push(value.to_string());
            } else {
                break;
            }
        }
    }
    let mut operators: VecDeque<_> = lines.last().unwrap().split_ascii_whitespace().collect();
    let mut cur_columns = vec![];
    for x in 0..buckets.len() {
        let cur_col_string = buckets[x].join("");
        let cur_col_string = cur_col_string.trim();
        // println!("{}", cur_col_string);
        if cur_col_string == "" {
            answer += use_operator_on(operators.pop_front().unwrap(), &cur_columns);
            cur_columns.clear();
        } else {
            cur_columns.push(cur_col_string.parse::<u64>().unwrap());
        }
    }
    answer += use_operator_on(operators.pop_front().unwrap(), &cur_columns);
    return Some(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
