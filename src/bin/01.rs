advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    let mut pos = 50;
    let mut answer = 0;
    for line in input.split('\n') {
        let (letter, num_string) = line.split_at_checked(1).unwrap();
        let amount = num_string.parse::<i64>().unwrap();
        let mult = match letter {
            "L" => -1,
            "R" => 1,
            _ => {
                panic!("first letter did not match");
            }
        };
        pos += amount * mult;
        pos = pos.rem_euclid(100);
        if pos == 0 {
            answer += 1;
        }
    }
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut pos: i64 = 50;
    let mut answer = 0;
    for line in input.split('\n') {
        let (letter, num_string) = line.split_at_checked(1).unwrap();
        let mut amount = num_string.parse::<i64>().unwrap();
        let mult = match letter {
            "L" => -1,
            "R" => 1,
            _ => {
                panic!("first letter did not match");
            }
        };
        while amount > 0 {
            let to_move = 1;
            pos += to_move * mult;
            pos = pos.rem_euclid(100);
            if pos == 0 {
                answer += 1;
            }
            amount -= to_move;
        }
    }
    Some(answer)
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
        assert_eq!(result, Some(6));
    }
}
