use std::collections::HashSet;

use advent_of_code::{find_char_coords, get_char_at_coord, Coords};
use rayon::iter::Positions;

advent_of_code::solution!(6);

#[derive(Clone, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone, Debug)]
struct Guard {
    x: i32,
    y: i32,
    direction: Direction,
}

fn simulate(input: &str, current_pos: &mut Guard) -> (HashSet<String>, HashSet<String>) {
    let mut visited_count: HashSet<String> = HashSet::new();
    let mut visited_directional: HashSet<String> = HashSet::new();

    loop {
        let id = format!(
            "{}_{}_{:?}",
            current_pos.x, current_pos.y, current_pos.direction
        );
        let id_short = format!("{}_{}", current_pos.x, current_pos.y,);
        if visited_directional.contains(&id) {
            break;
        }

        visited_directional.insert(id);
        visited_count.insert(id_short);

        let in_front = match current_pos.direction {
            Direction::UP => &Coords {
                x: current_pos.x,
                y: current_pos.y - 1,
            },
            Direction::DOWN => &Coords {
                x: current_pos.x,
                y: current_pos.y + 1,
            },
            Direction::LEFT => &Coords {
                x: current_pos.x - 1,
                y: current_pos.y,
            },
            Direction::RIGHT => &Coords {
                x: current_pos.x + 1,
                y: current_pos.y,
            },
        };

        let cc = get_char_at_coord(input, in_front, false);

        if cc.is_none() {
            break;
        }

        if cc.unwrap() == '#' {
            current_pos.direction = match current_pos.direction {
                Direction::UP => Direction::RIGHT,
                Direction::DOWN => Direction::LEFT,
                Direction::LEFT => Direction::UP,
                Direction::RIGHT => Direction::DOWN,
            };
            continue;
        } else {
            match current_pos.direction {
                Direction::UP => current_pos.y -= 1,
                Direction::DOWN => current_pos.y += 1,
                Direction::LEFT => current_pos.x -= 1,
                Direction::RIGHT => current_pos.x += 1,
            }
        }
    }
    return (visited_count, visited_directional);
}

pub fn part_one(input: &str) -> Option<u32> {
    let binding = find_char_coords(input, '^');
    let start_pos = binding.get(0).unwrap();

    let mut current_pos = Guard {
        x: start_pos.x,
        y: start_pos.y,
        direction: Direction::UP,
    };

    let (visited_count, visited_directional) = simulate(input, &mut current_pos);

    Some(visited_count.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let binding = find_char_coords(input, '^');
    let start_pos = binding.get(0).unwrap();

    let mut current_pos = Guard {
        x: start_pos.x,
        y: start_pos.y,
        direction: Direction::UP,
    };

    let (mut visited_count, initial_directional) = simulate(input, &mut current_pos);

    // manual remove first points
    let f1 = format!("{}_{}", start_pos.x, start_pos.y);
    visited_count.remove(&f1);
    let f2 = format!("{}_{}", start_pos.x, start_pos.y - 1);
    visited_count.remove(&f2);

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
