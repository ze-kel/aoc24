use std::collections::{HashMap, HashSet};

use advent_of_code::{CoordMap, Coords};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let map = CoordMap::new(input);

    let antennas: Vec<(&Coords, &char)> = map.iter().filter(|(_, v)| **v != '.').collect();

    let mut antennas_map: HashMap<char, HashSet<Coords>> = HashMap::new();

    for (coors, char) in antennas {
        antennas_map
            .entry(char.clone())
            .or_insert_with(HashSet::new)
            .insert(coors.clone());
    }

    let count = map
        .iter()
        .map(|(coords, _)| {
            let coverage = antennas_map.iter().any(|(_, a_points)| {
                if a_points.len() < 2 {
                    return false;
                }

                a_points.iter().any(|p1| {
                    if coords.same(p1) {
                        return false;
                    }
                    let dist = coords.distance(p1);
                    let slope = coords.slope(p1);

                    a_points.iter().any(|p2| {
                        if coords.same(p2) || p1.same(p2) {
                            return false;
                        }
                        let dist2 = coords.distance(p2);
                        let slope2 = coords.slope(p2);

                        slope == slope2 && dist * 2 as f64 == dist2
                    })
                })
            });
            if coverage {
                1
            } else {
                0
            }
        })
        .sum();

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = CoordMap::new(input);
    let mut vizmap = CoordMap::new(input);

    let antennas: Vec<(&Coords, &char)> = map.iter().filter(|(_, v)| **v != '.').collect();

    let mut antennas_map: HashMap<char, HashSet<Coords>> = HashMap::new();

    for (coors, char) in antennas {
        antennas_map
            .entry(char.clone())
            .or_insert_with(HashSet::new)
            .insert(coors.clone());
    }

    let count = map
        .iter()
        .map(|(coords, _)| {
            let coverage = antennas_map.iter().any(|(_, a_points)| {
                if a_points.len() < 2 {
                    return false;
                }

                if !a_points.contains(&coords) {
                    a_points.iter().any(|p1| {
                        let slope = coords.slope(p1);

                        a_points.iter().any(|p2| {
                            if p1.same(p2) {
                                return false;
                            }
                            let slope2 = coords.slope(p2);

                            slope == slope2
                        })
                    })
                } else {
                    a_points.iter().any(|p1| {
                        if p1.same(coords) {
                            return false;
                        }
                        return true;
                    })
                }
            });
            if coverage {
                vizmap.set(coords, '#');
                1
            } else {
                0
            }
        })
        .sum();

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
