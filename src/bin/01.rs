use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut left_array: Vec<i32> = vec![];
    let mut right_array: Vec<i32> = vec![];

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let (left, right) = line.split_once("   ").unwrap();
        left_array.push(left.parse::<i32>().unwrap());
        right_array.push(right.parse::<i32>().unwrap());
    }

    left_array.sort();
    right_array.sort();

    assert!(left_array.len() > 0, "left array is not empty");
    assert_eq!(left_array.len(), right_array.len(), "equal length");

    let mut acc = 0;

    for i in 0..left_array.len() {
        acc += (left_array[i] - right_array[i]).abs();
    }

    return Some(acc);
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut left_array: Vec<i32> = vec![];
    let mut right_map: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let (left, right) = line.split_once("   ").unwrap();
        left_array.push(left.parse::<i32>().unwrap());
        let right_val = right.parse::<i32>().unwrap();
        right_map.insert(right_val, right_map.get(&right_val).unwrap_or(&0) + 1);
    }

    left_array.sort();

    assert!(left_array.len() > 0, "left array is not empty");
    assert!(right_map.len() > 0, "right map is not empty");

    let mut acc = 0;

    for i in 0..left_array.len() {
        acc += left_array[i] * right_map.get(&left_array[i]).unwrap_or(&0)
    }

    return Some(acc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
