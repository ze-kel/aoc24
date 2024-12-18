use std::{collections::HashSet, fs};

use advent_of_code::{extract_numbers, wrap_number, CoordMap, Coords};

advent_of_code::solution!(14);

#[derive(Clone, Debug)]
struct Robot {
    x: i32,
    y: i32,
    vel_x: i32,
    vel_y: i32,
}

fn map_robots(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|l| {
            let n = extract_numbers(l);

            Robot {
                x: n[0] as i32,
                y: n[1] as i32,
                vel_x: n[2] as i32,
                vel_y: n[3] as i32,
            }
        })
        .collect()
}

fn move_robot(r: &Robot, time: i32, max_x: i32, max_y: i32) -> Robot {
    Robot {
        x: wrap_number(r.x + r.vel_x * time, 0, max_x),
        y: wrap_number(r.y + r.vel_y * time, 0, max_y),
        vel_x: r.vel_x,
        vel_y: r.vel_y,
    }
}

fn count_robots(rr: &Vec<Robot>, max_x: i32, max_y: i32) -> (u32, u32, u32, u32) {
    // X Y quadrants
    let mut q_0_0 = 0;
    let mut q_0_1 = 0;
    let mut q_1_0 = 0;
    let mut q_1_1 = 0;

    let x_mid = max_x / 2;
    let y_mid = max_y / 2;

    for item in rr {
        assert!(item.x >= 0);
        assert!(item.y >= 0);
        assert!(item.x <= max_x);
        assert!(item.y <= max_y);
        if item.x == x_mid || item.y == y_mid {
            continue;
        }

        let xp = item.x < x_mid;
        let yp = item.y < y_mid;

        match (xp, yp) {
            (true, true) => q_0_0 += 1,
            (true, false) => q_0_1 += 1,
            (false, true) => q_1_0 += 1,
            (false, false) => q_1_1 += 1,
        }
    }

    return (q_0_0, q_0_1, q_1_0, q_1_1);
}

pub fn part_one(input: &str) -> Option<u32> {
    // let (max_x, max_y) = (10, 6);
    let (max_x, max_y) = (100, 102);

    let robots = map_robots(input);

    let moved: Vec<Robot> = robots
        .iter()
        .map(|r| move_robot(r, 100, max_x, max_y))
        .collect();

    let (q, w, e, r) = count_robots(&moved, max_x, max_y);

    Some(q * w * e * r)
}

pub fn part_two(input: &str) -> Option<u32> {
    // let (max_x, max_y) = (10, 6);
    let (max_x, max_y) = (100, 102);

    let robots = map_robots(input);

    let mut vizes: Vec<String> = vec![];

    for i in 0..100000 {
        let moved: Vec<Robot> = robots
            .clone()
            .iter()
            .map(|r| move_robot(r, i, max_x, max_y))
            .collect();

        let mut hs: HashSet<Coords> = HashSet::new();

        for r in moved.clone() {
            hs.insert(Coords { x: r.x, y: r.y });
        }

        let x_symm_coeff: u32 = moved
            .iter()
            .map(|r| {
                if hs.contains(&Coords {
                    x: max_x - r.x,
                    y: r.y,
                }) {
                    1
                } else {
                    0
                }
            })
            .sum();

        let percent_symm = x_symm_coeff as f64 / (robots.len() as f64);

        if percent_symm > 0.2 {
            let mut map = CoordMap::new_from_map("");

            for r in moved {
                map.set(&Coords { x: r.x, y: r.y }, 'R');
            }

            let v = map.viz_to_string_raw(0, 0, max_x, max_y, &'.');

            vizes.extend(vec!["".to_owned(), i.to_string(), v]);
        }
    }

    let _ = fs::write("./1414.txt", vizes.join("\n"));

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
