use advent_of_code::extract_numbers;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(17);

#[derive(Clone, Debug)]
struct Computer {
    a: i64,
    b: i64,
    c: i64,

    output: Vec<i64>,

    instuctions: Vec<i64>,
    pointer: usize,
}

impl Computer {
    fn get_next(&mut self) -> Option<(i64, i64)> {
        let a = self.instuctions.get(self.pointer);
        let b = self.instuctions.get(self.pointer + 1);

        if a.is_none() {
            return None;
        }

        return Some((*a.unwrap(), *b.unwrap()));
    }

    fn get_combo(&mut self, v: i64) -> i64 {
        match v {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!(),
        }
    }

    fn division(&mut self, v: i64) -> i64 {
        let c = self.get_combo(v);
        let bottom = 2_i64.pow(c as u32);
        let vvv = self.a as f64 / bottom as f64;
        return vvv.floor() as i64;
    }

    fn instruction(&mut self, instr: i64, literal: i64) {
        match instr {
            0 => self.a = self.division(literal),
            1 => self.b = self.b ^ literal,
            2 => self.b = self.get_combo(literal) % 8,
            3 => {
                if self.a != 0 {
                    self.pointer = literal.try_into().unwrap();
                    return;
                }
            }
            4 => self.b = self.b ^ self.c,
            5 => {
                let vvv = self.get_combo(literal) % 8;
                self.output.push(vvv)
            }
            6 => self.b = self.division(literal),
            7 => self.c = self.division(literal),

            _ => panic!(),
        }

        self.pointer += 2;
    }

    fn run_to_end(&mut self) {
        loop {
            let ins = self.get_next();
            match ins {
                None => break,
                Some((i, v)) => self.instruction(i, v),
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let lines: Vec<&str> = input.split('\n').collect();
    let (one, two, three, five) = (&lines[0], &lines[1], &lines[2], &lines[4]);

    let mut comp = Computer {
        a: extract_numbers(*one)[0] as i64,
        b: extract_numbers(*two)[0] as i64,
        c: extract_numbers(*three)[0] as i64,
        output: vec![],
        instuctions: extract_numbers(*five).iter().map(|v| *v as i64).collect(),
        pointer: 0,
    };

    comp.run_to_end();

    print!("{:?}", comp.output);

    let j = comp
        .output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    Some(j)
}

fn divide_range(start: i64, end: i64, n: i64) -> Vec<(i64, i64)> {
    if n <= 0 {
        return vec![];
    }

    let range_size = (end - start) as f64;
    let chunk_size = (range_size / n as f64).ceil() as i64;

    (0..n)
        .map(|i| {
            let chunk_start = start + (i * chunk_size);
            let chunk_end = (start + ((i + 1) * chunk_size)).min(end);
            (chunk_start, chunk_end)
        })
        .collect()
}

#[derive(Clone, Debug)]
struct Chunk {
    start: i64,
    end: i64,
    coeff_start: i64,
}

pub fn part_two(input: &str) -> Option<i64> {
    let lines: Vec<&str> = input.split('\n').collect();

    let ins: Vec<i64> = extract_numbers(lines[4])
        .iter()
        .map(|v| *v as i64)
        .collect();

    let mut i = 1;

    let mut lower_bound = 0;
    let mut upper_bound = 0;

    let mut found_start = false;

    loop {
        i = i * 2;

        let mut c = Computer {
            a: i,
            b: 0,
            c: 0,
            instuctions: ins.clone(),
            pointer: 0,
            output: vec![],
        };

        c.run_to_end();

        if c.output.len() == c.instuctions.len() && !found_start {
            lower_bound = i / 2;
            found_start = true;
        }

        if c.output.len() > c.instuctions.len() && found_start {
            upper_bound = i;
            break;
        }
    }

    print!(
        "\n{} {} {}",
        lower_bound,
        upper_bound,
        upper_bound - lower_bound
    );

    let chunks = divide_range(lower_bound, upper_bound, 100000000);

    let mut v: Vec<Chunk> = chunks
        .par_iter()
        .map(|(s, e)| {
            let mut c1 = Computer {
                a: *s,
                b: 0,
                c: 0,
                instuctions: ins.clone(),
                pointer: 0,
                output: vec![],
            };

            c1.run_to_end();

            let mut c2 = Computer {
                a: *e,
                b: 0,
                c: 0,
                instuctions: ins.clone(),
                pointer: 0,
                output: vec![],
            };

            c2.run_to_end();

            Chunk {
                start: *s,
                end: *e,
                coeff_start: compare_vecs(&c1.output, &c1.instuctions)
                    + compare_vecs(&c2.output, &c2.instuctions),
            }
        })
        .collect();

    v.sort_by_key(|a| a.coeff_start);
    v.reverse();

    for ch in v {
        //  print!("\n CHUNK LEN {} {}", ch.end - ch.start, ch.coeff_start);

        let brute_force: Vec<i64> = (ch.start..ch.end)
            .into_par_iter()
            .filter(|&num| {
                let mut c = Computer {
                    a: num,
                    b: 0,
                    c: 0,
                    instuctions: ins.clone(),
                    pointer: 0,
                    output: vec![],
                };

                c.run_to_end();

                return c.instuctions == c.output;
            })
            .collect();

        //print!("\n CHUNK DONE\n");

        if brute_force.len() > 0 {
            // print!("\n\n\n ANSWER: {} \n\n\n", brute_force[0]);
            return Some(brute_force[0]);
        }
    }

    None
}

fn compare_vecs(a: &Vec<i64>, b: &Vec<i64>) -> i64 {
    a.iter()
        .enumerate()
        .map(|(v, i)| {
            if b.get(v) == Some(i) {
                return 1;
            } else {
                return 0;
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
