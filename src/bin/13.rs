use regex::Regex;

advent_of_code::solution!(13);

#[derive(Clone, Debug)]
struct Info {
    x: f64,
    y: f64,
}

#[derive(Clone, Debug)]
struct System {
    a: Info,
    b: Info,
    target: Info,
}

fn extract_numbers(text: &str, add: f64) -> Info {
    let re = Regex::new(r"-?\d+(?:\.\d+)?").unwrap();

    let n: Vec<f64> = re
        .find_iter(text)
        .filter_map(|m| m.as_str().parse::<f64>().ok())
        .collect();

    Info {
        x: n[0] + add,
        y: n[1] + add,
    }
}

fn solve(s: System) -> Option<(f64, f64)> {
    print!("\n{:?}", s);
    let a = [[s.a.x, s.b.x], [s.a.y, s.b.y]];

    let b = [s.target.x, s.target.y];

    let det = a[0][0] * a[1][1] - a[0][1] * a[1][0];

    if det == 0.0 {
        return None;
    }

    let inv_det = 1.0 / det;
    let x = (a[1][1] * b[0] - a[0][1] * b[1]) * inv_det;
    let y = (a[0][0] * b[1] - a[1][0] * b[0]) * inv_det;

    if x < 0.0 || y < 0.0 {
        return None;
    }

    let xr = x.round();
    let yr = y.round();

    let x_correct = s.a.x * xr + s.b.x * yr == s.target.x;
    let y_correct = s.a.y * xr + s.b.y * yr == s.target.y;

    if x_correct && y_correct {
        return Some((xr, yr));
    }

    None
}

pub fn part_one(input: &str) -> Option<f64> {
    let systems: Vec<System> = input
        .split("\n\n")
        .map(|sub| {
            let l: Vec<&str> = sub.lines().collect();

            System {
                a: extract_numbers(l[0], 0.0),
                b: extract_numbers(l[1], 0.0),
                target: extract_numbers(l[2], 0.0),
            }
        })
        .collect();

    let tokens: f64 = systems
        .iter()
        .map(|s| match solve(s.clone()) {
            Some((x, y)) => x * 3.0 + y,
            None => 0.0,
        })
        .sum();

    Some(tokens)
}

pub fn part_two(input: &str) -> Option<f64> {
    let systems: Vec<System> = input
        .split("\n\n")
        .map(|sub| {
            let l: Vec<&str> = sub.lines().collect();

            System {
                a: extract_numbers(l[0], 0.0),
                b: extract_numbers(l[1], 0.0),
                target: extract_numbers(l[2], 10000000000000.0),
            }
        })
        .collect();

    let tokens: f64 = systems
        .iter()
        .map(|s| match solve(s.clone()) {
            Some((x, y)) => x * 3.0 + y,
            None => 0.0,
        })
        .sum();

    Some(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480.0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
