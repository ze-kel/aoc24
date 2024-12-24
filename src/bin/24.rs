use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rand::Rng;
use serde::{Deserialize, Serialize};

advent_of_code::solution!(24);

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Operation {
    AND,
    OR,
    XOR,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Connection {
    computed: bool,

    op: Operation,

    left: String,
    right: String,
    out: String,
}

fn solve(connections: &mut Vec<Connection>, values: &mut HashMap<String, i8>) -> String {
    for item in connections.iter_mut() {
        item.computed = false
    }

    loop {
        let mut has_uncoputed = false;
        let mut changed = 0;

        for item in connections.iter_mut() {
            if item.computed {
                continue;
            }

            has_uncoputed = true;

            let l = values.get(&item.left);
            let r = values.get(&item.right);

            if l.is_some() && r.is_some() {
                let ll = l.unwrap();
                let rr = r.unwrap();

                let result = match item.op {
                    Operation::AND => *ll == 1 && *rr == 1,
                    Operation::OR => *ll == 1 || *rr == 1,
                    Operation::XOR => *ll != *rr,
                };

                let n = match result {
                    true => 1,
                    false => 0,
                };
                values.insert(item.out.clone(), n);
                item.computed = true;
                changed += 1;
            }
        }

        if !has_uncoputed || changed == 0 {
            break;
        }
    }

    let zv: Vec<&String> = values
        .keys()
        .filter(|v| v.starts_with('z'))
        .sorted()
        .rev()
        .collect();

    let zvv: Vec<&i8> = zv.iter().map(|k| values.get(*k).unwrap()).collect();

    let s = zvv.iter().join("");

    return s;
}

fn parse_initial(input: &str) -> (Vec<Connection>, HashMap<String, i8>) {
    let (base, conns) = input.split_once("\n\n").unwrap();

    let connections: Vec<Connection> = conns
        .lines()
        .map(|l| {
            let sp: Vec<&str> = l.split(" ").collect();

            Connection {
                computed: false,
                left: (**sp.get(0).unwrap()).to_owned(),
                op: match *sp.get(1).unwrap() {
                    "AND" => Operation::AND,
                    "OR" => Operation::OR,
                    "XOR" => Operation::XOR,
                    _ => todo!(),
                },
                right: (**sp.get(2).unwrap()).to_owned(),
                out: (**sp.get(4).unwrap()).to_owned(),
            }
        })
        .collect();

    let mut values: HashMap<String, i8> = HashMap::new();

    for l in base.lines() {
        let (target, value) = l.split_once(": ").unwrap();

        values.insert(target.to_string(), value.parse().unwrap());
    }

    (connections, values)
}

pub fn part_one(input: &str) -> Option<u128> {
    let (mut connections, mut values) = parse_initial(input);
    let s = solve(&mut connections, &mut values);

    Some(u128::from_str_radix(&s, 2).unwrap())
}

fn make_random_binary(len: u32) -> String {
    let mut acc: Vec<u8> = vec![];
    let mut rng = rand::thread_rng();

    for i in 0..len {
        let b: bool = rng.gen();

        if i == len - 1 {
            acc.push(1);
        } else {
            acc.push(match b {
                true => 1,
                false => 0,
            });
        }
    }

    acc.iter().join("")
}

fn add_binaries(a: &str, b: &str) -> String {
    let a = u128::from_str_radix(a, 2).unwrap();
    let b = u128::from_str_radix(b, 2).unwrap();

    let sum = a + b;
    format!("{:b}", sum)
}

fn build_values(a: &str, b: &str) -> HashMap<String, i8> {
    let mut hm: HashMap<String, i8> = HashMap::new();

    let l = a.len() - 1;

    for (i, char) in a.chars().enumerate() {
        hm.insert(format!("x{:02}", l - i), char.to_string().parse().unwrap());
    }

    for (i, char) in b.chars().enumerate() {
        hm.insert(format!("y{:02}", l - i), char.to_string().parse().unwrap());
    }

    hm
}

fn get_first_off(expected: &str, solved: &str) -> Option<usize> {
    for i in 0..solved.len() {
        let ep: i16 = expected.len() as i16 - 1 - i as i16;

        let sp: i16 = solved.len() as i16 - 1 - i as i16;

        let e = expected.chars().nth(ep.max(0) as usize);
        let s = solved.chars().nth(sp.max(0) as usize);

        if e != s {
            return Some(i + 1);
        }
    }

    Some(solved.len())
}

fn get_rating_off(
    expected: &str,
    values: &mut HashMap<String, i8>,
    connections: &mut Vec<Connection>,
) -> i32 {
    let solved = solve(connections, values);
    get_first_off(&expected, &solved).unwrap_or(0) as i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Swap {
    a: usize,
    b: usize,
    rating: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Checker {
    a: String,
    b: String,
    as_values: HashMap<String, i8>,
    expected: String,
}

fn get_rating_off_checkers(c: &mut Vec<Checker>, conns: &mut Vec<Connection>) -> i32 {
    let all: Vec<i32> = c
        .iter_mut()
        .map(|cc| get_rating_off(&cc.expected, &mut cc.as_values, conns))
        .collect();

    *all.iter().min().unwrap()
}

pub fn part_two(input: &str) -> Option<String> {
    /*
       I tried different ways of brute forcing it looking for shortcuts but did not find a reasonable one in multiple hours and attempts
       Ended up implementing other person solution in Rust https://github.com/piman51277/AdventOfCode/blob/master/solutions/2024/24/index2prog.js
    */

    let (connections, _) = parse_initial(input);

    let mut flags: HashSet<String> = HashSet::new();

    let fagate0s: Vec<&Connection> = connections
        .iter()
        .filter(|c| is_direct(c) && c.op == Operation::XOR)
        .collect();

    let fagate3s: Vec<&Connection> = connections
        .iter()
        .filter(|c| !is_direct(c) && c.op == Operation::XOR)
        .collect();

    for gate in fagate3s.iter() {
        if !is_output(gate) {
            flags.insert(gate.out.clone());
        }
    }

    let output_gates: Vec<&Connection> = connections.iter().filter(|v| is_output(v)).collect();

    for gate in output_gates.iter() {
        if gate.out == "z45" {
            if gate.op != Operation::OR {
                flags.insert(gate.out.clone());
            }
        } else if gate.op != Operation::XOR {
            flags.insert(gate.out.clone());
        }
    }

    let mut check_next: Vec<Connection> = vec![];

    for gate in fagate0s.iter() {
        if flags.contains(&gate.out) {
            continue;
        }

        if gate.out == "z00" {
            continue;
        }

        let matches: Vec<&&Connection> = fagate3s
            .iter()
            .filter(|v| v.left == gate.out || v.right == gate.out)
            .collect();

        if matches.len() == 0 {
            check_next.push((*gate).clone());
            flags.insert(gate.out.clone());
        }
    }

    for gate in check_next.iter() {
        let ln = gate.left.chars().skip(1).join("");

        let intended_result = format!("z{}", ln);

        let matches: Vec<&&Connection> = fagate3s
            .iter()
            .filter(|v| v.out == intended_result)
            .collect();

        assert!(matches.len() == 1);

        let mm = matches.get(0).unwrap();

        let tcha = mm.left.clone();
        let tchb = mm.right.clone();

        let or_matches: Vec<&Connection> = connections
            .iter()
            .filter(|v| v.op == Operation::OR && (tcha == v.out || tchb == v.out))
            .collect();

        assert!(or_matches.len() == 1);

        let mmm = or_matches.get(0).unwrap();

        if tcha != mmm.out {
            flags.insert(tcha);
        } else if tchb != mmm.out {
            flags.insert(tchb);
        } else {
            panic!("Wrong")
        }
    }

    assert!(flags.len() == 8);

    let mut vf: Vec<&String> = flags.iter().collect();
    vf.sort();

    Some(vf.iter().join(","))
}

fn is_direct(c: &Connection) -> bool {
    c.left.starts_with("x") || c.right.starts_with("x")
}

fn is_output(c: &Connection) -> bool {
    c.out.starts_with("z")
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }*/

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
