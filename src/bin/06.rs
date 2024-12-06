use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashSet;

use advent_of_code::{CoordMap, Coords};

advent_of_code::solution!(6);

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Guard {
    x: i32,
    y: i32,
    direction: Direction,
}

enum SimulationResult {
    Exit,
    Loop,
}

fn simulate(
    input: &CoordMap,
    current_pos: &mut Guard,
) -> (HashSet<Coords>, HashSet<Guard>, SimulationResult) {
    let mut visited_count: HashSet<Coords> = HashSet::new();
    let mut visited_directional: HashSet<Guard> = HashSet::new();

    loop {
        if visited_directional.contains(&current_pos) {
            return (visited_count, visited_directional, SimulationResult::Loop);
        }

        visited_directional.insert(current_pos.clone());
        visited_count.insert(Coords {
            x: current_pos.x,
            y: current_pos.y,
        });

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

        let cc = input.get(in_front);

        if cc.is_none() {
            return (visited_count, visited_directional, SimulationResult::Exit);
        }

        if cc.unwrap().clone() == '#' {
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
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = CoordMap::new(input);

    let binding = map.find('^');
    let start_pos = binding.get(0).unwrap();

    let mut current_pos = Guard {
        x: start_pos.x,
        y: start_pos.y,
        direction: Direction::UP,
    };

    let (visited_count, _, _) = simulate(&map, &mut current_pos);

    Some(visited_count.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = CoordMap::new(input);
    let binding = map.find('^');
    let start_pos = binding.get(0).unwrap();

    let start_pos = Guard {
        x: start_pos.x,
        y: start_pos.y,
        direction: Direction::UP,
    };

    let (mut visited_count, _, _) = simulate(&map, &mut start_pos.clone());

    // manual remove first points
    visited_count.remove(&Coords {
        x: start_pos.x,
        y: start_pos.y,
    });
    visited_count.remove(&Coords {
        x: start_pos.x,
        y: start_pos.y - 1,
    });

    let cnt = visited_count
        .par_iter()
        .map(|cc| {
            let mut updated_map = map.clone();
            updated_map.set(&cc, '#');

            let (_, _, res) = simulate(&updated_map, &mut start_pos.clone());

            return match res {
                SimulationResult::Exit => 0,
                SimulationResult::Loop => 1,
            };
        })
        .sum();

    Some(cnt)
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
        assert_eq!(result, Some(6));
    }
}
