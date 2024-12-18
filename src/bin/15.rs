use std::collections::HashSet;

use advent_of_code::{CoordMap, Coords};

advent_of_code::solution!(15);

#[derive(Clone, Debug, Eq, PartialEq)]
enum Move {
    Right,
    Down,
    Up,
    Left,
}

fn create_map(input: &str) -> (CoordMap, Vec<Move>) {
    let spl: Vec<&str> = input.split("\n\n").collect();

    let map = CoordMap::new_from_map(&spl[0]);

    (
        map,
        spl[1]
            .trim()
            .replace("\n", "")
            .chars()
            .enumerate()
            .map(|(_, l)| match l {
                '^' => Move::Up,
                '<' => Move::Left,
                '>' => Move::Right,
                'v' => Move::Down,
                letter => {
                    panic!("unexpected {letter}")
                }
            })
            .collect(),
    )
}

fn get_shifted(c: &Coords, m: &Move) -> Coords {
    match m {
        Move::Right => Coords { x: c.x + 1, y: c.y },
        Move::Down => Coords { x: c.x, y: c.y + 1 },
        Move::Up => Coords { x: c.x, y: c.y - 1 },
        Move::Left => Coords { x: c.x - 1, y: c.y },
    }
}

fn process_command(map: &mut CoordMap, from: &Coords, mv: &Move) -> Option<Vec<(char, Coords)>> {
    let mut to_move = vec![('@', from.clone())];
    let mut do_move = true;

    loop {
        let (_, c) = &to_move.last().unwrap();
        let next = get_shifted(c, &mv);

        let next_on_map = map.get(&next);

        match next_on_map {
            Some('#') => {
                do_move = false;
                break;
            }
            Some('.') => {
                break;
            }
            Some('O') => {
                to_move.push(('O', next));
            }
            Some(_) => todo!(),
            None => todo!(),
        }
    }

    if !do_move {
        return None;
    }

    Some(to_move)
}

fn find_score(map: &CoordMap) -> u32 {
    let boxes = map.find_char('O');

    let sum: i32 = boxes.iter().map(|c| c.x + c.y * 100).sum();

    sum as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, commands) = create_map(&input);

    for mv in commands {
        let cloned_map = map.clone();
        let robot = cloned_map.find_char('@')[0];
        let to_move = process_command(&mut map, &robot, &mv);

        if let Some(mut tm) = to_move {
            tm.reverse();
            for (character, coords) in tm {
                let becomes = get_shifted(&coords, &mv);
                map.set(&becomes, character);
                map.set(&coords, '.');
            }
        }
    }

    Some(find_score(&map))
}

fn process_command_2(
    map: &mut CoordMap,
    from: &Coords,
    mv: &Move,
    from_adj: bool,
) -> Option<Vec<(char, Coords)>> {
    let from_char = map.get(from).unwrap().clone();
    //print!("\nprocess command {:?} {from_char}", from);

    let mut to_move = vec![(from_char, from.clone())];
    let mut do_move = true;

    let mut block_to_adj = from_adj;

    loop {
        let tmc = to_move.clone();
        let (char, c) = tmc.last().unwrap();

        if !block_to_adj && (*mv == Move::Up || *mv == Move::Down) && (*char == '[' || *char == ']')
        {
            let adj_on = match char {
                '[' => Move::Right,
                ']' => Move::Left,
                _ => todo!(),
            };

            let adjacent = get_shifted(&c, &adj_on);

            let ajd_move_result = process_command_2(map, &adjacent, mv, true);

            match ajd_move_result {
                Some(a) => {
                    for item in a {
                        to_move.push(item);
                    }
                }
                None => {
                    do_move = false;
                    break;
                }
            }
        }

        let next = get_shifted(c, &mv);

        let next_on_map = map.get(&next);

        match next_on_map {
            Some('#') => {
                do_move = false;
                break;
            }
            Some('.') => {
                break;
            }
            Some('[') => {
                to_move.push(('[', next));
            }
            Some(']') => {
                to_move.push((']', next));
            }

            Some(_) => todo!(),
            None => todo!(),
        }
        block_to_adj = false
    }

    if !do_move {
        return None;
    }

    Some(to_move)
}

fn find_score_2(map: &CoordMap) -> u32 {
    let boxes = map.find_char('[');

    let sum: i32 = boxes.iter().map(|c| c.x + c.y * 100).sum();

    sum as u32
}

pub fn part_two(input: &str) -> Option<u32> {
    let inp = input
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.");

    let (mut map, commands) = create_map(&inp);

    map.viz('.');

    for mv in commands {
        let cloned_map = map.clone();
        let robot = cloned_map.find_char('@')[0];
        let to_move = process_command_2(&mut map, &robot, &mv, false);

        let mut was_set: HashSet<Coords> = HashSet::new();

        if let Some(tm) = to_move {
            for (character, coords) in tm {
                let becomes = get_shifted(&coords, &mv);
                map.set(&becomes, character);
                was_set.insert(becomes.clone());
                if !was_set.contains(&coords) {
                    map.set(&coords, '.');
                }
            }
        }
    }

    Some(find_score_2(&map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
