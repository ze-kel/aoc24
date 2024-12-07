use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy)]
enum Operators {
    MULT,
    ADD,
    COMB,
}

impl Operators {
    pub fn vars_1() -> Vec<Operators> {
        return vec![Operators::MULT, Operators::ADD];
    }
    pub fn vars_2() -> Vec<Operators> {
        return vec![Operators::MULT, Operators::ADD, Operators::COMB];
    }
}

fn generate_op_combinations(length: usize, ops: Vec<Operators>) -> Vec<Vec<Operators>> {
    if length == 0 {
        return vec![vec![]];
    }

    let minus_combinations = generate_op_combinations(length - 1, ops.clone());
    let mut result = Vec::new();

    for combo in minus_combinations {
        for op in ops.clone() {
            let mut new_combo = combo.clone();
            new_combo.push(op.clone());
            result.push(new_combo);
        }
    }

    result
}

fn eval_pairs(numbers: Vec<u64>, ops: Vec<Operators>) -> u64 {
    let mut acc = numbers.get(0).unwrap().clone();

    for n in 0..ops.len() {
        let next = numbers.get(n + 1).unwrap();
        match ops.get(n).unwrap() {
            Operators::MULT => acc = acc * next,
            Operators::ADD => acc = acc + next,
            Operators::COMB => acc = format!("{}{}", acc, next).parse().unwrap(),
        }
    }
    acc
}

fn solver(input: &str, part2: bool) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();

    let result = lines
        .par_iter()
        .map(|line| {
            let spl: Vec<&str> = line.split(": ").collect();

            let target: u64 = spl.get(0).unwrap().parse().unwrap();
            let nums: Vec<u64> = spl
                .get(1)
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            let options = generate_op_combinations(
                nums.len() - 1,
                match part2 {
                    true => Operators::vars_2(),
                    false => Operators::vars_1(),
                },
            );

            let found = options
                .iter()
                .any(|opt| eval_pairs(nums.clone(), opt.clone()) == target);

            if found {
                target
            } else {
                0
            }
        })
        .sum();

    Some(result)
}

pub fn part_one(input: &str) -> Option<u64> {
    solver(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    solver(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
