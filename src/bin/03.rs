use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<i64> {
    let re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    let mut acc = 0;
    for caps in re.captures_iter(input) {
        let numone: i64 = (&caps[1]).parse().unwrap();
        let numtwo: i64 = (&caps[2]).parse().unwrap();
        println!("{} {} * {}", &caps[0], &caps[1], &caps[2]);
        acc += numone * numtwo;
    }

    Some(acc)
}

pub fn part_two(input: &str) -> Option<i64> {
    let remul = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();
    let reinstr = Regex::new(r"do(n't)?\(\)").unwrap();

    let mut pos = 0;

    let mut enabled = true;
    let mut acc = 0;
    let mut ignored = 0;

    while pos < input.len() {
        let mult_m = remul.captures_at(input, pos);
        let inst_m = reinstr.captures_at(input, pos);

        match (mult_m, inst_m) {
            (None, None) => return Some(acc),
            (None, Some(inrt)) => {
                if inrt.get(0).unwrap().as_str() == "do()" {
                    enabled = true
                } else {
                    assert_eq!(inrt.get(0).unwrap().as_str(), "don't()");
                    enabled = false
                }
                pos = inrt.get(0).unwrap().start()
            }
            (Some(v), None) => {
                if enabled {
                    let a: i64 = v.get(1).unwrap().as_str().parse().unwrap();
                    let b: i64 = v.get(2).unwrap().as_str().parse().unwrap();
                    acc += a * b
                } else {
                    ignored += 1
                }

                pos = v.get(0).unwrap().start()
            }
            (Some(v), Some(inrt)) => {
                let v_pos = v.get(0).unwrap().start();
                let i_pos = inrt.get(0).unwrap().start();

                if v_pos < i_pos {
                    if enabled {
                        let a: i64 = v.get(1).unwrap().as_str().parse().unwrap();
                        let b: i64 = v.get(2).unwrap().as_str().parse().unwrap();
                        acc += a * b
                    } else {
                        ignored += 1
                    }

                    pos = v.get(0).unwrap().start()
                } else {
                    if inrt.get(0).unwrap().as_str() == "do()" {
                        enabled = true
                    } else {
                        assert_eq!(inrt.get(0).unwrap().as_str(), "don't()");
                        enabled = false
                    }

                    pos = inrt.get(0).unwrap().start()
                }
            }
        }
        pos += 1
    }

    assert_ne!(ignored, 0);

    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
