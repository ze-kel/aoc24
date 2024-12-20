use std::collections::{hash_map::Iter, HashMap};

use regex::Regex;

pub mod template;

// Use this file to add helper functions and additional modules.

fn divide_range(start: i64, end: i64, n: i64) -> Vec<(i64, i64)> {
    if n <= 0 {
        return vec![];
    }

    let range_size = (end - start) as f64;
    let chunk_size = (range_size / n as f64).ceil() as i64;

    (0..n)
        .map(|i| {
            let chunk_start = start + (i * chunk_size);
            let chunk_end = (start + ((i + 1) * chunk_size)).min(end);
            (chunk_start, chunk_end)
        })
        .collect()
}

pub fn extract_numbers(text: &str) -> Vec<f64> {
    let re = Regex::new(r"-?\d+(?:\.\d+)?").unwrap();

    let n: Vec<f64> = re
        .find_iter(text)
        .filter_map(|m| m.as_str().parse::<f64>().ok())
        .collect();

    return n;
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Coords {
    pub fn distance(&self, other: &Coords) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }
    pub fn path_distance(&self, other: &Coords) -> u32 {
        let d = (self.x - other.x).abs() + (self.y - other.y).abs();
        d.try_into().unwrap()
    }
    pub fn same(&self, other: &Coords) -> bool {
        self.x == other.x && self.y == other.y
    }
    pub fn slope(&self, other: &Coords) -> f64 {
        if (other.x as f64 - self.x as f64).abs() < f64::EPSILON {
            f64::INFINITY
        } else {
            (other.y as f64 - self.y as f64) / (other.x as f64 - self.x as f64)
        }
    }
}

pub fn are_coords_on_line(a: &Coords, b: &Coords, c: &Coords) -> bool {
    (b.y - a.y) * (c.x - a.x) == (c.y - a.y) * (b.x - a.x)
}

pub fn wrap_number(number: i32, min: i32, max: i32) -> i32 {
    if min >= max {
        panic!("min must be less than max");
    }

    let range = max - min + 1;
    if number < min {
        max - ((min - number - 1) % range)
    } else if number > max {
        min + ((number - max - 1) % range)
    } else {
        number
    }
}

pub fn find_char_coords(input: &str, target: char) -> Vec<Coords> {
    let mut res: Vec<Coords> = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == target {
                res.push(Coords {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                });
            }
        }
    }

    return res;
}

pub fn get_char_at_coord(text: &str, cord: &Coords, wrapping: bool) -> Option<char> {
    let y_wrapped = wrap_number(
        cord.y.try_into().unwrap(),
        0,
        (text.lines().count() - 1).try_into().unwrap(),
    );

    let x_wrapped = wrap_number(
        cord.x.try_into().unwrap(),
        0,
        (text.lines().count() - 1).try_into().unwrap(),
    );

    if !wrapping && (y_wrapped != cord.y || x_wrapped != cord.x) {
        return None;
    }

    let line = text.lines().nth(y_wrapped.try_into().unwrap()).unwrap();
    let char = line.chars().nth(x_wrapped.try_into().unwrap()).unwrap();
    return Some(char);
}

#[derive(Clone, Debug)]
pub struct CoordMap {
    map: HashMap<Coords, char>,

    pub x_len: i32,
    pub y_len: i32,
}

impl CoordMap {
    pub fn new_max(x_max: i32, y_max: i32) -> CoordMap {
        return CoordMap {
            map: HashMap::new(),
            y_len: y_max + 1,
            x_len: x_max + 1,
        };
    }

    pub fn new_len(x_len: i32, y_len: i32) -> CoordMap {
        return CoordMap {
            map: HashMap::new(),
            y_len: y_len,
            x_len: x_len,
        };
    }

    pub fn new_from_map(input: &str) -> CoordMap {
        let lines: Vec<&str> = input.lines().collect();
        let mut c = CoordMap {
            map: HashMap::new(),
            y_len: lines.len() as i32,
            x_len: lines[0].len() as i32,
        };
        if input.is_empty() {
            return c;
        }
        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                c.set(
                    &Coords {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    },
                    char,
                );
            }
        }
        return c;
    }

    pub fn set(&mut self, k: &Coords, v: char) {
        self.map.insert(k.clone(), v);
    }

    pub fn get(&self, k: &Coords) -> Option<&char> {
        self.map.get(k)
    }

    pub fn find_char(&self, cc: char) -> Vec<&Coords> {
        return self
            .map
            .iter()
            .filter(|(_, v)| **v == cc)
            .map(|(k, _)| k)
            .collect();
    }

    pub fn iter(&self) -> Iter<'_, Coords, char> {
        return self.map.iter();
    }

    pub fn viz_to_string_raw(
        &self,
        min_x: i32,
        min_y: i32,
        max_x: i32,
        max_y: i32,
        empty: &char,
    ) -> String {
        let mut str = "".to_owned();

        str += "\n";
        for y in min_y..max_y {
            for x in min_x..max_x {
                str += &format!("{}", self.get(&Coords { x: x, y: y }).unwrap_or(empty));
            }
            str += "\n";
        }
        str += "\n";

        return str;
    }

    pub fn viz_raw(&self, min_x: i32, min_y: i32, max_x: i32, max_y: i32, empty: &char) {
        print!(
            "{}",
            self.viz_to_string_raw(min_x, min_y, max_x, max_y, empty)
        );
    }

    pub fn viz(&self, empty: char) {
        print!("{}", self.viz_to_string(&empty));
    }

    pub fn viz_to_string(&self, empty: &char) -> String {
        return self.viz_to_string_raw(0, 0, self.x_len, self.y_len, empty);
    }

    pub fn get_adjacent_xy(&self, c: &Coords) -> Vec<Coords> {
        vec![
            Coords { x: c.x + 1, y: c.y },
            Coords { x: c.x - 1, y: c.y },
            Coords { x: c.x, y: c.y + 1 },
            Coords { x: c.x, y: c.y - 1 },
        ]
    }

    pub fn get_adjacent_diag(&self, c: &Coords) -> Vec<Coords> {
        vec![
            Coords {
                x: c.x + 1,
                y: c.y + 1,
            },
            Coords {
                x: c.x + 1,
                y: c.y - 1,
            },
            Coords {
                x: c.x - 1,
                y: c.y + 1,
            },
            Coords {
                x: c.x - 1,
                y: c.y - 1,
            },
        ]
    }

    pub fn coord_exists(&self, c: &Coords) -> bool {
        self.get(c).is_some()
    }

    pub fn best_to_pos(
        &self,
        from: &Coords,
        to: &Coords,
        can_step_on: fn(Option<&char>) -> bool,
    ) -> HashMap<Coords, u32> {
        let mut best_steps_to_pos: HashMap<Coords, u32> = HashMap::new();

        let mut q = vec![(from.clone(), 0)];

        while q.len() > 0 {
            let (position, steps) = q.pop().unwrap();

            let best = best_steps_to_pos.get(&position);

            if best.is_some() && steps >= *best.unwrap() {
                continue;
            } else {
                best_steps_to_pos.insert(position.clone(), steps);
            }

            if position == *to {
                continue;
            }

            let adj = self.get_adjacent_xy(&position);
            let possible: Vec<&Coords> = adj
                .iter()
                .filter(|c| {
                    if c.x < 0 || c.y < 0 || c.x >= self.x_len || c.y >= self.y_len {
                        return false;
                    }

                    let best = best_steps_to_pos.get(c);

                    if best.is_some() && steps + 1 >= *best.unwrap() {
                        return false;
                    }
                    let letter = self.get(c);
                    return can_step_on(letter);
                })
                .collect();

            for item in possible {
                q.push((item.clone(), steps + 1));
            }
        }

        best_steps_to_pos
    }

    pub fn shortest_steps(
        &self,
        from: &Coords,
        to: &Coords,
        can_step_on: fn(Option<&char>) -> bool,
    ) -> Option<u32> {
        let best_steps_to_pos = self.best_to_pos(from, to, can_step_on);
        best_steps_to_pos.get(to).copied()
    }
}
