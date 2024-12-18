use std::{char, collections::HashSet};

use advent_of_code::{CoordMap, Coords};

advent_of_code::solution!(12);

fn find_borders(map: &CoordMap, points: &HashSet<Coords>, ch: &char) -> HashSet<(Coords, Coords)> {
    let mut borders: HashSet<(Coords, Coords)> = HashSet::new();

    for point in points {
        let adj: Vec<Coords> = map.get_adjacent_xy(&point);

        let only_diff: Vec<&Coords> = adj.iter().filter(|x| map.get(x) != Some(ch)).collect();

        for item in only_diff {
            borders.insert(((*point).clone(), item.clone()));
        }
    }

    borders
}

#[derive(Clone, Debug)]
struct Zone {
    char: char,
    points: HashSet<Coords>,
}

fn find_zones(map: &CoordMap) -> Vec<Zone> {
    let mut found_zone: HashSet<Coords> = HashSet::new();

    let mut zones: Vec<Zone> = vec![];

    for (pos, char) in map.iter() {
        if found_zone.contains(pos) {
            continue;
        }

        let mut zone = Zone {
            char: char.clone(),
            points: HashSet::new(),
        };

        let mut q: Vec<Coords> = vec![pos.clone()];

        while q.len() > 0 {
            let c = q.pop().unwrap();

            zone.points.insert(c.clone());
            found_zone.insert(c.clone());

            let adj = map.get_adjacent_xy(&c);
            let next: Vec<&Coords> = adj
                .iter()
                .filter(|x| map.get(x) == Some(char) && !found_zone.contains(x))
                .collect();

            for item in next {
                q.push(item.clone());
            }
        }

        zones.push(zone);
    }

    zones
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = CoordMap::new_from_map(input);
    let zones = find_zones(&map);

    let mut acc: u32 = 0;

    for zone in zones {
        let area = zone.points.len();
        let perimiter = find_borders(&map, &zone.points, &zone.char).len();
        acc += area as u32 * perimiter as u32;
    }

    Some(acc)
}

fn sort_borders_into_sides(
    borders: &HashSet<(Coords, Coords)>,
    map: &CoordMap,
) -> Vec<Vec<(Coords, Coords)>> {
    let mut sorted: Vec<Vec<(Coords, Coords)>> = vec![];
    let mut taken: HashSet<(Coords, Coords)> = HashSet::new();

    for border in borders {
        if taken.contains(&border) {
            continue;
        }

        taken.insert(border.clone());

        let mut side: Vec<(Coords, Coords)> = vec![border.clone()];

        let mut q: Vec<(Coords, Coords)> = vec![border.clone()];

        while q.len() > 0 {
            let item = q.pop().unwrap();

            let (n1, n2) = item.clone();

            assert!(map.get(&n1) != map.get(&n2));

            let is_vertical = n1.y == n2.y;

            let adjacent = match is_vertical {
                true => vec![
                    (
                        Coords {
                            x: n1.x,
                            y: n1.y + 1,
                        },
                        Coords {
                            x: n2.x,
                            y: n2.y + 1,
                        },
                    ),
                    (
                        Coords {
                            x: n1.x,
                            y: n1.y - 1,
                        },
                        Coords {
                            x: n2.x,
                            y: n2.y - 1,
                        },
                    ),
                ],
                false => vec![
                    (
                        Coords {
                            x: n1.x + 1,
                            y: n1.y,
                        },
                        Coords {
                            x: n2.x + 1,
                            y: n2.y,
                        },
                    ),
                    (
                        Coords {
                            x: n1.x - 1,
                            y: n1.y,
                        },
                        Coords {
                            x: n2.x - 1,
                            y: n2.y,
                        },
                    ),
                ],
            };

            for second in borders {
                if taken.contains(&second) {
                    continue;
                }

                let (s1, s2) = second;

                let is_adjacent = adjacent.iter().any(|pair| {
                    (*pair == (s1.clone(), s2.clone())
                        && map.get(&pair.0) == map.get(s1)
                        && map.get(&pair.1) == map.get(s2))
                        || (*pair == (s2.clone(), s1.clone())
                            && map.get(&pair.0) == map.get(s1)
                            && map.get(&pair.1) == map.get(s2))
                });

                if is_adjacent {
                    side.push(second.clone());
                    q.push(second.clone());
                    taken.insert(second.clone());
                }
            }
        }

        sorted.push(side);
    }

    assert!(taken.len() == borders.len());

    sorted
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = CoordMap::new_from_map(input);
    let zones = find_zones(&map);

    let mut acc: u32 = 0;

    for zone in zones {
        let area = zone.points.len();
        let borders = find_borders(&map, &zone.points, &zone.char.clone());

        let sorted = sort_borders_into_sides(&borders, &map);

        let perimiter = sorted.len();

        assert!(perimiter != 0);

        acc += area as u32 * perimiter as u32;
    }

    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
