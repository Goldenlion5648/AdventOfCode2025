use std::collections::HashSet;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut invalid_total = 0;
    for range in input.trim().split(',') {
        range.split_once('-').into_iter().for_each(|(lo, hi)| {
            let lo = lo.parse::<u64>().unwrap();
            let hi = hi.parse::<u64>().unwrap();
            for id in lo..=hi {
                let cur_id_as_string = id.to_string();
                if cur_id_as_string.len() % 2 != 0 {
                    continue;
                }
                let (first, second) = cur_id_as_string
                    .split_at_checked(cur_id_as_string.len() / 2)
                    .unwrap();
                if first == second {
                    invalid_total += id;
                }
            }
        });
    }
    Some(invalid_total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut invalid_total = 0;
    for range in input.trim().split(',') {
        range.split_once('-').into_iter().for_each(|(lo, hi)| {
            let lo = lo.parse::<u64>().unwrap();
            let hi = hi.parse::<u64>().unwrap();
            for id in lo..=hi {
                let cur_id_as_string = id.to_string();
                for size in 1..=(cur_id_as_string.len() / 2) {
                    let mult = cur_id_as_string.len() / size;
                    let part_to_repeat = cur_id_as_string.chars().take(size).collect::<String>();
                    if part_to_repeat.repeat(mult) == cur_id_as_string {
                        invalid_total += id;
                        break;
                    }
                }
            }
        });
    }
    Some(invalid_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
