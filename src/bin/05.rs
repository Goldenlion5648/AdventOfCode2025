use std::ops::Range;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ingredients) = input.trim().split_once("\n\n").unwrap();
    let mut ranges = ranges
        .trim()
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            Range {
                start: start.parse::<u64>().unwrap(),
                end: end.parse::<u64>().unwrap() + 1,
            }
        })
        .collect::<Vec<Range<u64>>>();
    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let ingredients = ingredients
        .trim()
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    // let new_order = vec![];
    let mut answer = 0;
    for ingredient in ingredients {
        for range in ranges.as_slice() {
            if range.contains(&ingredient) {
                answer += 1;
                break;
            }
        }
    }

    return Some(answer);
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = input.trim().split_once("\n\n").unwrap();
    let mut ranges = ranges
        .trim()
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            Range {
                start: start.parse::<u64>().unwrap(),
                end: end.parse::<u64>().unwrap() + 1,
            }
        })
        .collect::<Vec<Range<u64>>>();
    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut new_order = vec![ranges[0].clone()];
    let mut answer = 0;
    for range in ranges.as_slice() {
        if new_order.last().unwrap().contains(&range.start) {
            let last = new_order.len() - 1;
            new_order[last].end = std::cmp::max(new_order[new_order.len() - 1].end, range.end)
        } else {
            new_order.push(range.clone());
        }
    }
    for range in new_order {
        answer += range.end - range.start;
    }

    return Some(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
