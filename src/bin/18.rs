use advent_of_code::{extract_numbers, CoordMap, Coords};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    //let (max, b) = (6, 12);
    let (max, b) = (70, 1024);

    let mut map = CoordMap::new_max(max, max);
    for (i, line) in input.lines().enumerate() {
        if i >= b {
            break;
        }
        let c = extract_numbers(line);

        map.set(
            &Coords {
                x: c[0] as i32,
                y: c[1] as i32,
            },
            '#',
        );
    }

    map.shortest_steps(&Coords { x: 0, y: 0 }, &Coords { x: max, y: max }, |c| {
        c == None
    })
}

pub fn part_two(input: &str) -> Option<String> {
    //let (max, _) = (6, 12);
    let (max, _) = (70, 1024);

    let lines: Vec<&str> = input.lines().collect();
    let ll = lines.len();

    let mut range = (0, ll);

    while range.1 - range.0 > 1 {
        let mid = (range.1 - range.0) / 2;

        let mut map = CoordMap::new_max(max, max);
        for (i, line) in input.lines().enumerate() {
            if i >= range.0 + mid {
                break;
            }
            let c = extract_numbers(line);

            map.set(
                &Coords {
                    x: c[0] as i32,
                    y: c[1] as i32,
                },
                '#',
            );
        }

        map.viz('.');

        let v = map.shortest_steps(&Coords { x: 0, y: 0 }, &Coords { x: max, y: max }, |c| {
            c == None
        });

        if v.is_none() {
            range = (range.0, mid + range.0);
        } else {
            range = (mid + range.0, range.1);
        }
    }

    let a = lines[range.0];

    Some(a.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_owned()));
    }
}
