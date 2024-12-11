use std::collections::HashMap;

advent_of_code::solution!(11);

fn split(s: &u128) -> Vec<u128> {
    let ss: String = s.to_string();

    match s {
        0 => vec![1],
        _ if ss.len() % 2 == 0 => {
            let (fst, scnd) = ss.split_at(ss.len() / 2);

            vec![fst.parse().unwrap(), scnd.parse().unwrap()]
        }

        n => vec![2024 * n],
    }
}

pub fn part_one(input: &str) -> Option<u128> {
    let stones: Vec<u128> = input
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let mut cache: SplitCacheVec = HashMap::new();

    let ss = find_split_len(stones.clone(), 25, &mut cache);

    Some(ss)
}

type SplitCacheVec = HashMap<(Vec<u128>, u32), u128>;

fn find_split_len(s: Vec<u128>, times: u32, c: &mut SplitCacheVec) -> u128 {
    let a = c.get(&(s.clone(), times));

    if a.is_some() {
        return a.unwrap().clone();
    }

    let result: u128 = match times {
        1 => s
            .iter()
            .map(|ss| {
                let lll: u128 = split(ss).len().try_into().unwrap();
                lll
            })
            .sum(),
        _ => s
            .iter()
            .map(|ss| find_split_len(split(ss), times - 1, c))
            .sum(),
    };

    c.insert((s.clone(), times), result);

    result
}

pub fn part_two(input: &str) -> Option<u128> {
    let stones: Vec<u128> = input
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let mut cache: SplitCacheVec = HashMap::new();

    let ss = find_split_len(stones.clone(), 75, &mut cache);

    Some(ss)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
