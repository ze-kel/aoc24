use std::collections::{HashMap, HashSet};

use advent_of_code::{CoordMap, Coords};

advent_of_code::solution!(16);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    EastRight, // right
    WestLeft,  // left
    SouthDown, // down
    NorthUp,   // up
}

fn move_direction(c1: &Coords, c2: &Coords) -> Direction {
    if c1.x == c2.x {
        if c1.y > c2.y {
            return Direction::NorthUp;
        } else {
            return Direction::SouthDown;
        }
    } else {
        if c1.x > c2.x {
            return Direction::WestLeft;
        } else {
            return Direction::EastRight;
        }
    }
}

fn calc_direction_turn_mult(d1: &Direction, d2: &Direction) -> u32 {
    match (d1, d2) {
        (Direction::EastRight, Direction::WestLeft) => 2,
        (Direction::EastRight, Direction::SouthDown) => 1,
        (Direction::EastRight, Direction::NorthUp) => 1,

        (Direction::WestLeft, Direction::EastRight) => 2,
        (Direction::WestLeft, Direction::SouthDown) => 1,
        (Direction::WestLeft, Direction::NorthUp) => 1,

        (Direction::SouthDown, Direction::EastRight) => 1,
        (Direction::SouthDown, Direction::WestLeft) => 1,
        (Direction::SouthDown, Direction::NorthUp) => 2,

        (Direction::NorthUp, Direction::EastRight) => 1,
        (Direction::NorthUp, Direction::WestLeft) => 1,
        (Direction::NorthUp, Direction::SouthDown) => 2,

        (_, _) => 0,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Position {
    position: Coords,
    direction: Direction,
    score: u32,
    visited: HashSet<Coords>,
}

fn solver(input: &str, store_visited: bool) -> (u32, Vec<Position>) {
    let map = CoordMap::new_from_map(input);

    let start = map.find_char('S');

    let mut q = vec![Position {
        position: start[0].clone(),
        direction: Direction::EastRight,
        score: 0,
        visited: HashSet::new(),
    }];

    let mut best_score_to_pos: HashMap<(Coords, Direction), u32> = HashMap::new();
    let mut finished: Vec<Position> = vec![];
    let mut min_score: u32 = 1000000000;

    while q.len() > 0 {
        let mut item = q.pop().unwrap();

        let best = best_score_to_pos.get(&(item.position.clone(), item.direction.clone()));

        if best.is_some() && item.score > *best.unwrap() {
            continue;
        } else {
            best_score_to_pos.insert((item.position.clone(), item.direction.clone()), item.score);
        }

        if store_visited {
            item.visited.insert(item.position.clone());
        }

        if map.get(&item.position) == Some(&'E') {
            if item.score < min_score {
                min_score = item.score
            }
            finished.push(item);
            continue;
        }

        if item.score > min_score {
            continue;
        }

        let possible: Vec<Position> = map
            .get_adjacent_xy(&item.position)
            .iter()
            .filter_map(|c| {
                let direction = move_direction(&item.position, c);
                let direction_mult = calc_direction_turn_mult(&item.direction, &direction);
                if direction_mult == 2 {
                    return None;
                }

                let letter = map.get(c);
                assert!(letter != None);
                if letter == Some(&'#') {
                    return None;
                }

                let score = item.score + 1 + direction_mult * 1000;
                if score > min_score {
                    return None;
                }

                let nv = match store_visited {
                    false => HashSet::new(),
                    true => item.visited.clone(),
                };

                Some(Position {
                    position: c.clone(),
                    direction: direction,
                    score: score,
                    visited: nv,
                })
            })
            .collect();

        for item in possible {
            q.push(item);
        }
    }

    (min_score, finished)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (m, _) = solver(input, false);
    Some(m)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (m, finished) = solver(input, true);

    let only_best: Vec<&Position> = finished.iter().filter(|v| v.score == m).collect();

    let mut uniq: HashSet<Coords> = HashSet::new();

    for item in only_best {
        for v in item.visited.iter() {
            uniq.insert(v.clone());
        }
    }

    Some(uniq.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_directgion() {
        let right = move_direction(&Coords { x: 10, y: 10 }, &Coords { x: 11, y: 10 });
        let left = move_direction(&Coords { x: 10, y: 10 }, &Coords { x: 9, y: 10 });
        let down = move_direction(&Coords { x: 10, y: 10 }, &Coords { x: 10, y: 11 });
        let up = move_direction(&Coords { x: 10, y: 10 }, &Coords { x: 10, y: 9 });

        assert_eq!(right, Direction::EastRight);
        assert_eq!(left, Direction::WestLeft);
        assert_eq!(down, Direction::SouthDown);
        assert_eq!(up, Direction::NorthUp);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
