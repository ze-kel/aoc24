use std::collections::HashMap;

pub mod template;

// Use this file to add helper functions and additional modules.

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
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

pub struct CoordMap {
    map: HashMap<Coords, char>,
}

impl CoordMap {
    pub fn new(&mut self, input: &str) -> &CoordMap {
        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                self.set(
                    Coords {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    },
                    char,
                );
            }
        }
        self
    }
    pub fn set(&mut self, k: Coords, v: char) {
        self.map.insert(k, v);
    }

    pub fn get(&mut self, k: Coords) -> Option<&char> {
        self.map.get(&k)
    }
}
