use std::{collections::HashMap, u32};

use advent_of_code::{extract_numbers, CoordMap, Direction};
use itertools::Itertools;

advent_of_code::solution!(21);

const NUMERIC_MAP: &str = "789
456
123
X0A";

const ROBOT_MAP: &str = "X^A
<v>";

fn find_possible_presses(
    from_char: char,
    to_char: char,
    field: &CoordMap,
    add_a: bool,
) -> Vec<String> {
    let from = field.find_char(from_char)[0];
    let to = field.find_char(to_char)[0];

    let dist_x = to.x - from.x;
    let dist_y = to.y - from.y;

    let x_char = match dist_x > 0 {
        true => ">",
        false => "<",
    };

    let y_char = match dist_y > 0 {
        true => "v",
        false => "^",
    };

    let s: String = x_char.repeat(dist_x.abs() as usize) + &y_char.repeat(dist_y.abs() as usize);

    let p1 = s
        .chars()
        .permutations((dist_x.abs() + dist_y.abs()) as usize);

    let mut perms: Vec<String> = p1
        .map(|p| p.into_iter().collect())
        .unique()
        .filter(|v: &String| {
            let mut current = from.clone();

            for char in v.chars() {
                current = current.move_direction(match char {
                    '>' => Direction::Right(1),
                    '<' => Direction::Left(1),
                    '^' => Direction::Up(1),
                    'v' => Direction::Down(1),
                    _ => todo!(),
                });

                let ff = field.get(&current);

                if ff == None || ff == Some(&'X') {
                    return false;
                }
            }

            return true;
        })
        .collect();

    if add_a {
        for item in perms.iter_mut() {
            item.push('A');
        }
    }

    perms
}

fn find_possible_presses_line(line: &str, map: &CoordMap) -> Vec<Vec<String>> {
    let chars: Vec<char> = line.chars().collect();
    line.chars()
        .enumerate()
        .map(|(i, c)| {
            find_possible_presses(
                match i {
                    0 => 'A',
                    _ => chars[i - 1],
                },
                c,
                &map,
                true,
            )
        })
        .multi_cartesian_product()
        .map(|combo| combo.iter().map(|s| (*s).clone()).collect())
        .collect()
}

fn find_line_shortest(
    line: &str,
    depth_left: u32,
    depth_start: u32,
    num_map: &CoordMap,
    robot_map: &CoordMap,
    cache: &mut CacheHash,
) -> u64 {
    let cached = cache.get(&(line.to_string(), depth_left));

    if cached.is_some() {
        return *cached.unwrap();
    }

    let poss: Vec<Vec<String>> = find_possible_presses_line(
        line,
        match depth_left == depth_start {
            true => num_map,
            false => robot_map,
        },
    );

    let mut min = u64::MAX;

    for itm in poss {
        let ll = match depth_left <= 1 {
            true => itm.join("").len() as u64,
            false => itm
                .iter()
                .map(|j| {
                    find_line_shortest(j, depth_left - 1, depth_start, num_map, robot_map, cache)
                })
                .sum(),
        };

        assert!(ll != 0);

        if ll < min {
            min = ll
        }
    }

    cache.insert((line.to_string(), depth_left), min);

    min as u64
}

type CacheHash = HashMap<(String, u32), u64>;

pub fn part_one(input: &str) -> Option<u64> {
    let num_map = CoordMap::new_from_map(NUMERIC_MAP);
    let robot_map = CoordMap::new_from_map(ROBOT_MAP);

    let mut cache: CacheHash = HashMap::new();

    let result = input
        .lines()
        .map(|line| {
            let shortest = find_line_shortest(line, 3, 3, &num_map, &robot_map, &mut cache);
            let number = extract_numbers(line)[0] as u64;

            shortest * number
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let num_map = CoordMap::new_from_map(NUMERIC_MAP);
    let robot_map = CoordMap::new_from_map(ROBOT_MAP);
    let mut cache: CacheHash = HashMap::new();

    Some(
        input
            .lines()
            .map(|line| {
                let shortest = find_line_shortest(line, 26, 26, &num_map, &robot_map, &mut cache);
                let number = extract_numbers(line)[0] as u64;

                shortest * number
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
