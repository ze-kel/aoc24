use std::collections::HashSet;

use advent_of_code::{CoordMap, Coords};

advent_of_code::solution!(25);

fn is_matching(lock: &CoordMap, key: &CoordMap) -> bool {
    lock.iter().all(|(coord, letter)| {
        let opposing = key.get(coord);

        match letter {
            '#' => opposing == Some(&'.'),
            '.' => true,
            _ => todo!(),
        }
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let strs: Vec<CoordMap> = input
        .split("\n\n")
        .map(|v| CoordMap::new_from_map(v))
        .collect();

    let mut overlaps: HashSet<(usize, usize)> = HashSet::new();

    for (n1, item1) in strs.iter().enumerate() {
        let is_key = *item1.get(&Coords { x: 0, y: 0 }).unwrap() == '#';

        if !is_key {
            continue;
        }

        for (n2, item2) in strs.iter().enumerate() {
            if n1 == n2 {
                continue;
            }

            let is_m = is_matching(item1, item2);

            if is_m {
                overlaps.insert((n1, n2));
            }
        }
    }

    Some(overlaps.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
