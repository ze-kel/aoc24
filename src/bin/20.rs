use std::collections::HashSet;

use advent_of_code::{CoordMap, Coords};

advent_of_code::solution!(20);

fn solver(input: &str, cheat_max: u32, min_save: u32) -> Option<u32> {
    let map = CoordMap::new_from_map(input);

    let start = map.find_char('S')[0];
    let end = map.find_char('E')[0];

    let no_cheating = map.best_to_pos(start, end, |c| c == Some(&'.') || c == Some(&'E'));
    let no_cheating_from_end = map.best_to_pos(end, start, |c| c == Some(&'.') || c == Some(&'S'));

    let base_path = *no_cheating.get(&end).unwrap();

    let mut saves: HashSet<(Coords, Coords)> = HashSet::new();

    for fstart in no_cheating {
        for fend in no_cheating_from_end.clone() {
            let dist = fstart.0.path_distance(&fend.0);

            if dist > cheat_max || dist <= 1 {
                continue;
            }

            let path_len = fstart.1 + fend.1 + dist;

            /*
            print!(
                "\n good dist {dist} s {:?} e {:?} from start {} from end {} total {} baseline {}",
                fstart.0, fend.0, fstart.1, fend.1, path_len, base_path
            );*/

            if path_len < base_path && base_path - path_len >= min_save {
                saves.insert((fstart.0.clone(), fend.0.clone()));
            }
        }
    }

    Some(saves.len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    //let (cheat_max, min_save) = (2, 1);
    let (cheat_max, min_save) = (2, 100);
    solver(input, cheat_max, min_save)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (cheat_max, min_save) = (20, 100);
    solver(input, cheat_max, min_save)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
