use std::collections::{hash_map::Iter, HashMap};

pub mod template;

// Use this file to add helper functions and additional modules.

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
}

impl CoordMap {
    pub fn new(input: &str) -> CoordMap {
        let mut c = CoordMap {
            map: HashMap::new(),
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

    pub fn find(&self, cc: char) -> Vec<&Coords> {
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

    pub fn viz(&self, min_x: i32, min_y: i32, max_x: i32, max_y: i32, empty: &char) {
        print!("\n");
        for y in min_y..max_y {
            for x in min_x..max_x {
                print!("{}", self.get(&Coords { x: x, y: y }).unwrap_or(empty))
            }
            print!("\n");
        }
        print!("\n");
    }
}
