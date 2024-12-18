use core::str;
use std::collections::HashSet;

use advent_of_code::{CoordMap, Coords};

advent_of_code::solution!(10);

fn is_next_number(a: &char, b: &char) -> bool {
    let int1: u32 = a.to_digit(10).unwrap();
    let int2: u32 = b.to_digit(10).unwrap();

    return int1 + 1 == int2;
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = CoordMap::new_from_map(input);

    let count: usize = map
        .find_char('0')
        .into_iter()
        .map(|s| {
            let mut positions = vec![s.clone()];

            let mut finished: HashSet<Coords> = HashSet::new();

            while positions.len() > 0 {
                let p = positions.pop().unwrap();
                let p_val = map.get(&p).unwrap();

                let near = map.get_adjacent_xy(&p);

                assert!(near.len() > 0);

                let filtered_next: Vec<&Coords> = near
                    .iter()
                    .filter(|p| map.coord_exists(p))
                    .filter(|p| is_next_number(p_val, map.get(p).unwrap()))
                    .collect();

                for item in filtered_next {
                    match map.get(item).unwrap() {
                        '9' => {
                            finished.insert(item.clone());
                        }
                        _ => positions.push(item.clone()),
                    }
                }
            }

            return finished.len();
        })
        .sum();

    return Some(count as u32);
}

struct PathBuilder {
    c: Coords,
    path: Vec<Coords>,
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = CoordMap::new_from_map(input);

    let count: usize = map
        .find_char('0')
        .into_iter()
        .map(|s| {
            let mut positions = vec![PathBuilder {
                c: s.clone(),
                path: vec![],
            }];

            let mut finished: HashSet<String> = HashSet::new();

            while positions.len() > 0 {
                let mut p = positions.pop().unwrap();
                p.path.push(p.c.clone());
                let p_val = map.get(&p.c).unwrap();

                let near = map.get_adjacent_xy(&p.c);

                assert!(near.len() > 0);

                let filtered_next: Vec<&Coords> = near
                    .iter()
                    .filter(|p| map.coord_exists(p))
                    .filter(|p| is_next_number(p_val, map.get(p).unwrap()))
                    .collect();

                for item in filtered_next {
                    match map.get(item).unwrap() {
                        '9' => {
                            p.path.push(item.clone());
                            let path_hashes: Vec<String> =
                                p.path.iter().map(|v| format!("{}-{}", v.x, v.y)).collect();
                            finished.insert(path_hashes.join("_"));
                        }
                        _ => positions.push(PathBuilder {
                            c: item.clone(),
                            path: p.path.clone(),
                        }),
                    }
                }
            }

            return finished.len();
        })
        .sum();

    return Some(count as u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
