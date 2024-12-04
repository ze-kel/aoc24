use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(4);

#[derive(Clone, Debug)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

fn wrap_number(number: i32, min: i32, max: i32) -> i32 {
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

// was 100% sure second part would be xmas with wraps lol
fn get_char_at_coord(text: &str, cord: &Coords, wrapping: bool) -> Option<char> {
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

fn find_char_coords(input: &str, target: char) -> Vec<Coords> {
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

fn check_all_possible_xmas(input: &str, x: &Coords) -> i32 {
    let mut count = 0;

    let options: Vec<Vec<Coords>> = vec![
        // Horizontal
        vec![
            x.clone(),
            Coords { x: x.x + 1, y: x.y },
            Coords { x: x.x + 2, y: x.y },
            Coords { x: x.x + 3, y: x.y },
        ],
        vec![
            x.clone(),
            Coords { x: x.x - 1, y: x.y },
            Coords { x: x.x - 2, y: x.y },
            Coords { x: x.x - 3, y: x.y },
        ],
        // Vertical
        vec![
            x.clone(),
            Coords { x: x.x, y: x.y + 1 },
            Coords { x: x.x, y: x.y + 2 },
            Coords { x: x.x, y: x.y + 3 },
        ],
        vec![
            x.clone(),
            Coords { x: x.x, y: x.y - 1 },
            Coords { x: x.x, y: x.y - 2 },
            Coords { x: x.x, y: x.y - 3 },
        ],
        // Diagonal
        vec![
            x.clone(),
            Coords {
                x: x.x + 1,
                y: x.y + 1,
            },
            Coords {
                x: x.x + 2,
                y: x.y + 2,
            },
            Coords {
                x: x.x + 3,
                y: x.y + 3,
            },
        ],
        vec![
            x.clone(),
            Coords {
                x: x.x - 1,
                y: x.y - 1,
            },
            Coords {
                x: x.x - 2,
                y: x.y - 2,
            },
            Coords {
                x: x.x - 3,
                y: x.y - 3,
            },
        ],
        vec![
            x.clone(),
            Coords {
                x: x.x - 1,
                y: x.y + 1,
            },
            Coords {
                x: x.x - 2,
                y: x.y + 2,
            },
            Coords {
                x: x.x - 3,
                y: x.y + 3,
            },
        ],
        vec![
            x.clone(),
            Coords {
                x: x.x + 1,
                y: x.y - 1,
            },
            Coords {
                x: x.x + 2,
                y: x.y - 2,
            },
            Coords {
                x: x.x + 3,
                y: x.y - 3,
            },
        ],
    ];

    for o in options {
        if check_x_mas(
            input,
            o.get(0).unwrap(),
            o.get(1).unwrap(),
            o.get(2).unwrap(),
            o.get(3).unwrap(),
        ) {
            count += 1
        }
    }
    return count;
}

fn check_x_mas(input: &str, x: &Coords, m: &Coords, a: &Coords, s: &Coords) -> bool {
    return get_char_at_coord(input, x, false) == Some('X')
        && get_char_at_coord(input, m, false) == Some('M')
        && get_char_at_coord(input, a, false) == Some('A')
        && get_char_at_coord(input, s, false) == Some('S');
}

fn check_x_mas_x(
    input: &str,
    c: &Coords,
    tr: &Coords,
    bl: &Coords,
    tl: &Coords,
    br: &Coords,
) -> bool {
    let center = get_char_at_coord(input, c, false);
    let top_right = get_char_at_coord(input, tr, false);
    let bot_left = get_char_at_coord(input, bl, false);
    let top_left = get_char_at_coord(input, tl, false);
    let bot_right = get_char_at_coord(input, br, false);

    let hor1 = (top_left == Some('M') && bot_right == Some('S'))
        || (top_left == Some('S') && bot_right == Some('M'));

    let hor2 = (top_right == Some('M') && bot_left == Some('S'))
        || (top_right == Some('S') && bot_left == Some('M'));

    return center == Some('A') && hor1 && hor2;
}

pub fn part_one(input: &str) -> Option<i32> {
    let all_x = find_char_coords(input, 'X');

    let v = all_x
        .par_iter()
        .map(|xxx| check_all_possible_xmas(input, xxx))
        .sum();

    Some(v)
}

pub fn part_two(input: &str) -> Option<u32> {
    let all_a = find_char_coords(input, 'A');

    let cnt = all_a
        .par_iter()
        .map(|a| {
            let tr = &Coords {
                x: a.x + 1,
                y: a.y - 1,
            };
            let tl = &Coords {
                x: a.x - 1,
                y: a.y - 1,
            };
            let br = &Coords {
                x: a.x + 1,
                y: a.y + 1,
            };
            let bl = &Coords {
                x: a.x - 1,
                y: a.y + 1,
            };
            if check_x_mas_x(input, &a, tr, bl, tl, br) {
                1
            } else {
                0
            }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_wrap() {
        // Basic wrapping
        assert_eq!(5, wrap_number(-1, 0, 5));
        assert_eq!(0, wrap_number(0, 0, 5));
        assert_eq!(1, wrap_number(1, 0, 5));
        assert_eq!(2, wrap_number(2, 0, 5));
        assert_eq!(3, wrap_number(3, 0, 5));
        assert_eq!(4, wrap_number(4, 0, 5));
        assert_eq!(5, wrap_number(5, 0, 5));
        assert_eq!(0, wrap_number(6, 0, 5));
    }
}
