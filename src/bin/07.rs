use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<_> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut cur_poses = HashSet::new();
    cur_poses.insert(lines[0].iter().position(|x| *x == 'S').unwrap());

    let mut answer = 0;
    for (y, line) in lines.iter().enumerate() {
        if y == 0 {
            continue;
        }
        let mut next_poses = HashSet::new();
        for prev in cur_poses {
            if line[prev] == '^' {
                next_poses.insert(prev - 1);
                next_poses.insert(prev + 1);
                answer += 1;
            } else {
                next_poses.insert(prev);
            }
        }
        cur_poses = next_poses.clone();
    }
    return Some(answer);
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<_> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut cur_poses = HashSet::new();
    cur_poses.insert((0, lines[0].iter().position(|x| *x == 'S').unwrap()));

    let mut dp = HashMap::new();
    dp.insert(*cur_poses.iter().next().unwrap(), 1);
    'outer: loop {
        let mut next_poses = HashSet::new();
        for (y, x) in &cur_poses {
            if *y == lines.len() {
                break 'outer;
            }
            let mut new_spots = vec![];
            if lines[*y][*x] == '^' {
                next_poses.insert((y + 1, x - 1));
                next_poses.insert((y + 1, x + 1));
                new_spots.push((y + 1, x - 1));
                new_spots.push((y + 1, x + 1));
                for new_spot in new_spots {
                    *dp.entry(new_spot).or_default() += dp[&(*y, *x)];
                }
            } else {
                next_poses.insert((y + 1, *x));
                new_spots.push((y + 1, *x));
                for new_spot in new_spots {
                    *dp.entry(new_spot).or_default() += dp[&(*y, *x)];
                }
            }
        }
        cur_poses = next_poses;
    }

    let mut total = 0;
    for key in &cur_poses {
        total += dp[&key];
    }

    return Some(total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
