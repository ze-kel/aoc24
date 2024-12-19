use std::collections::HashMap;

advent_of_code::solution!(19);

fn can_match(pattern: &str, towels: &Vec<&str>, memo: &mut HashMap<String, bool>) -> bool {
    let po = pattern.to_owned();

    if memo.contains_key(&po) {
        return false;
    }

    if po.len() == 0 {
        return true;
    }

    towels.iter().any(|t| {
        if po.starts_with(t) {
            let r = po.replacen(*t, "", 1);

            let fm = can_match(&r, towels, memo);

            if fm {
                return true;
            } else {
                memo.insert(po.clone(), false);
            }
        }
        false
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let spl: Vec<&str> = input.split("\n\n").collect();

    let towels: Vec<&str> = spl[0].split(", ").collect();

    let patterns: Vec<&str> = spl[1].lines().collect();

    let s: u32 = patterns
        .iter()
        .map(|p| {
            if can_match(p, &towels, &mut HashMap::new()) {
                1
            } else {
                0
            }
        })
        .sum();

    Some(s)
}

fn count_match(pattern: &str, towels: &Vec<&str>, memo: &mut HashMap<String, u128>) -> u128 {
    let po = pattern.to_owned();

    if po.len() == 0 {
        return 1;
    }

    let count = towels
        .iter()
        .map(|t| {
            if po.starts_with(t) {
                let r = po.replacen(*t, "", 1);

                let v = memo.get(&r);

                let res = match v {
                    Some(vv) => *vv,
                    None => {
                        let fm = count_match(&r, towels, memo);
                        memo.insert(r.clone(), fm);
                        return fm;
                    }
                };
                return res;
            }
            return 0;
        })
        .sum();

    return count;
}

pub fn part_two(input: &str) -> Option<u128> {
    let spl: Vec<&str> = input.split("\n\n").collect();

    let towels: Vec<&str> = spl[0].split(", ").collect();

    let patterns: Vec<&str> = spl[1].lines().collect();

    let mut mm: HashMap<String, u128> = HashMap::new();

    let s: u128 = patterns
        .iter()
        .map(|p| {
            let one = count_match(p, &towels, &mut mm);

            return one;
        })
        .sum();

    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
