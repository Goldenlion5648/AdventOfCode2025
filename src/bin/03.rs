use std::{cmp::max, collections::HashMap};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut answer: u64 = 0;
    for line in input.split('\n') {
        let mut highest_left = 0;
        let mut highest_overall: u64 = 0;
        for digit in line.chars() {
            let cur_val = digit.to_digit(10).unwrap() as u64;
            highest_overall = max(highest_overall, highest_left * 10 + cur_val);
            highest_left = max(highest_left, cur_val);
        }
        answer += highest_overall;
    }
    Some(answer)
}

pub fn get_remaining(pos: usize,  remaining: u32, full_string: &Vec<u64>, cache: &mut HashMap<(usize, u32), u64>) -> u64 {
    let key = (pos, remaining);
    if let Some(&found) = cache.get(&key) {
        return found;
    }
    if remaining == 0 {
        return 0;
    }
    if pos >= full_string.len() {
        return 0;
    }
    let mut ret = get_remaining(pos + 1, remaining, &full_string, cache);
    for i in pos..full_string.len() {
        if (remaining as usize + i) > full_string.len() {
            break;
        }
        ret = max(
            ret,
            full_string[i] * (10 as u64).pow(remaining - 1)
            + get_remaining(i + 1, remaining - 1, &full_string, cache),
        );
    }
    
    cache.insert(key, ret);
    return ret;
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut answer = 0;
    let mut starting_cache: HashMap<(usize, u32), u64> = HashMap::new();
    for line in input.trim().lines() {
        let line_as_vec = line.chars().map(|x| x.to_digit(10).unwrap() as u64).collect::<Vec<_>>();
        answer += get_remaining(0, 12, &line_as_vec, &mut starting_cache);
        starting_cache.clear();
    }
    return Some(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
