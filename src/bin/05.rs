use std::{cmp::Ordering, collections::HashSet};

advent_of_code::solution!(5);

fn common_solution(input: &str, second: bool) -> Option<u32> {
    let ss: Vec<&str> = input.split("\n\n").collect();

    let sort_order = ss[0];
    let n_line = ss[1];

    let mut nums: HashSet<i32> = HashSet::new();

    let mut rules: HashSet<String> = HashSet::new();

    for line in sort_order.lines() {
        let spl: Vec<&str> = line.split("|").collect();

        let left: i32 = spl[0].parse().unwrap();
        let right: i32 = spl[0].parse().unwrap();

        rules.insert(line.to_string());
        nums.insert(left);
        nums.insert(right);
    }

    let mut acc: u32 = 0;

    for line in n_line.lines() {
        let mut spl: Vec<&str> = line.split(",").collect();

        spl.sort_by(|a, b| {
            if rules.contains(&format!("{}|{}", a, b)) {
                return Ordering::Less;
            }

            if rules.contains(&format!("{}|{}", b, a)) {
                return Ordering::Greater;
            }

            return Ordering::Equal;
        });

        let joined = spl.join(",");

        if joined == line && !second || (joined != line && second) {
            let middle: u32 = spl.get(spl.len() / 2).unwrap().parse().unwrap();
            acc += middle
        }
    }

    Some(acc)
}

pub fn part_one(input: &str) -> Option<u32> {
    common_solution(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    common_solution(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
