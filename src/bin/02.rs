advent_of_code::solution!(2);

fn check_line(numbers: Vec<i32>) -> Result<bool, u32> {
    let is_decreasing = numbers.first().unwrap() > numbers.last().unwrap();

    for i in 0..(numbers.len() - 1) {
        let a = numbers[i];
        let b = numbers[i + 1];
        let diff = (a - b).abs();

        if (is_decreasing && b > a) || (!is_decreasing && a > b) || diff > 3 || diff < 1 {
            return Err((i).try_into().unwrap());
        }
    }
    return Ok(true);
}

fn processor(input: &str, fail_allowed: bool) -> Option<u32> {
    let mut safe = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let res = check_line(numbers.clone());

        match res {
            Ok(_) => {
                safe += 1;
            }
            Err(index) => {
                if fail_allowed {
                    let mut numbers_a = numbers.clone();
                    let mut numbers_b = numbers.clone();
                    numbers_a.remove(index.try_into().unwrap());
                    numbers_b.remove((index + 1).try_into().unwrap());

                    let res1 = check_line(numbers_a);
                    let res2 = check_line(numbers_b);
                    if res1.is_ok() || res2.is_ok() {
                        safe += 1;
                    }
                }
            }
        }
    }

    return Some(safe);
}

pub fn part_one(input: &str) -> Option<u32> {
    processor(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    processor(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
