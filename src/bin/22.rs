use std::{collections::HashMap, i32};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(22);

fn mix(value: u64, secret: u64) -> u64 {
    value ^ secret
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn next_number(mut secret: u64) -> u64 {
    secret = prune(mix(64 * secret, secret));
    let d1 = (secret as f64 / 32.0).floor();
    secret = prune(mix(d1 as u64, secret));
    secret = prune(mix(secret * 2048, secret));
    secret
}

fn n_number(mut s: u64, n: u64) -> u64 {
    for _ in 0..n {
        s = next_number(s);
    }
    s
}

pub fn part_one(input: &str) -> Option<u64> {
    let v: u64 = input
        .lines()
        .map(|v| {
            let s: u64 = v.parse().unwrap();

            n_number(s, 2000)
        })
        .sum();

    Some(v)
}

fn prices(mut s: u64, n: u64) -> Vec<i32> {
    let mut p: Vec<i32> = vec![];
    for _ in 0..n {
        p.push((s % 10) as i32);
        s = next_number(s);
    }
    p
}

fn diffs(prices: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];

    for nn in 1..prices.len() {
        let d: i32 = prices[nn] - prices[nn - 1];

        res.push(d);
    }

    res
}

fn precompute_monkey(prices: &Vec<i32>, d: &Vec<i32>) -> HashMap<Vec<i32>, i32> {
    let mut res: HashMap<Vec<i32>, i32> = HashMap::new();

    for i in 3..d.len() {
        let s = vec![d[i - 3], d[i - 2], d[i - 1], d[i]];
        let v = prices[i + 1];

        if !res.contains_key(&s) {
            res.insert(s, v);
        }
    }

    res
}

pub fn part_two(input: &str) -> Option<i32> {
    let monkeys: Vec<HashMap<Vec<i32>, i32>> = input
        .lines()
        .map(|v| {
            let s: u64 = v.parse().unwrap();
            let pp = prices(s, 2001);
            let dd = diffs(pp.clone());
            precompute_monkey(&pp, &dd)
        })
        .collect();

    let digits: Vec<i32> = (-9..=9).collect();
    let perms: Vec<Vec<i32>> = digits
        .iter()
        .combinations_with_replacement(4)
        .flat_map(|combo| combo.into_iter().map(|&v| v).permutations(4))
        .collect();

    let a = perms
        .par_iter()
        .map(|item| {
            monkeys
                .iter()
                .map(|m| m.get(item).unwrap_or(&0))
                .sum::<i32>()
        })
        .max();

    Some(a.unwrap() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_test() {
        assert_eq!(mix(15, 42), 37);

        assert_eq!(prune(100000000), 16113920);

        let pp = prices(123, 10);
        let dd = diffs(pp.clone());

        let targ = vec![-1, -1, 0, 2];

        let precomp = precompute_monkey(&pp, &dd);

        let v = precomp.get(&targ);

        assert!(v.is_some());
        assert_eq!(v, Some(&6));
    }

    /*

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    } */

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
