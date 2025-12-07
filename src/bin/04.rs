advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.trim().lines().collect();
    let mut answer = 0;
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let mut count = 0;
            if lines[y].chars().nth(x).unwrap() != '@' {
                continue;
            }
            for dy in (-1 as isize)..=1 {
                for dx in (-1 as isize)..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let cur_y = (y as i32) + (dy as i32);
                    let cur_x = (x as i32) + (dx as i32);
                    if (0..(lines.len() as i32)).contains(&cur_y)
                        && (0..(lines[y].len() as i32)).contains(&cur_x)
                    {
                        if lines[cur_y as usize].chars().nth(cur_x as usize).unwrap() == '@' {
                            count += 1;
                        }
                    }
                }
            }
            if count <= 3 {
                answer += 1
            }
        }
    }
    return Some(answer);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut answer = 0;
    let mut changed = true;
    while changed {
        changed = false;
        let mut to_remove = vec![];
        for y in 0..lines.len() {
            for x in 0..lines[y].len() {
                let mut count = 0;
                if lines[y][x] != '@' {
                    continue;
                }
                for dy in (-1 as isize)..=1 {
                    for dx in (-1 as isize)..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let cur_y = (y as i32) + (dy as i32);
                        let cur_x = (x as i32) + (dx as i32);
                        if (0..(lines.len() as i32)).contains(&cur_y)
                            && (0..(lines[y].len() as i32)).contains(&cur_x)
                        {
                            if lines[cur_y as usize][cur_x as usize] == '@' {
                                count += 1;
                            }
                        }
                    }
                }
                if count <= 3 {
                    answer += 1;
                    to_remove.push((y, x));
                    changed = true;
                }
            }
        }
        for (y, x) in to_remove {
            lines[y][x] = '.';
        }
    }

    return Some(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("{}", result.unwrap());
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
