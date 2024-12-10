advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy)]
struct Space {
    pub id: i32,
    pub places: i32,
}

#[derive(Debug, Clone, Copy)]
enum TakenSpace {
    Empty(Space),
    Number(Space),
}

fn makerow(input: &str) -> Vec<TakenSpace> {
    // I manually deleted empty line from row
    let mut counter = 0;
    let row: Vec<TakenSpace> = input
        .split("")
        .filter(|x| !x.trim().is_empty())
        .enumerate()
        .map(|(n, i)| {
            let i_parsed: i32 = i.parse().unwrap();

            if n % 2 == 0 {
                let v = TakenSpace::Number(Space {
                    id: counter,
                    places: i_parsed,
                });
                counter += 1;
                return v;
            } else {
                return TakenSpace::Empty(Space {
                    id: 0,
                    places: i_parsed,
                });
            }
        })
        .collect();

    row
}

fn shift_row_p1(row: &mut Vec<TakenSpace>) {
    let mut pointer_start = 0;
    let mut pointer_end = row.len() - 1;

    //row_viz(&row);

    while pointer_end > pointer_start {
        let start_item = row[pointer_start];
        let end_item = row[pointer_end];

        match (start_item, end_item) {
            (TakenSpace::Empty(mut space_empty), TakenSpace::Number(mut space_number)) => {
                if space_empty.places == space_number.places {
                    row[pointer_start] = end_item.clone();
                    row.remove(pointer_end);
                    pointer_start += 1;
                    pointer_end -= 1;
                    continue;
                }

                if space_empty.places > space_number.places {
                    space_empty.places -= space_number.places;
                    row[pointer_start] = TakenSpace::Empty(space_empty);
                    row.remove(pointer_end);

                    row.insert(
                        pointer_start,
                        TakenSpace::Number(Space {
                            places: space_number.places,
                            id: space_number.id,
                        }),
                    );
                    // Add to becuase we inserted element before
                    pointer_start += 1;
                    // End is unchanged becuase remove is -1 and insert is +1
                    continue;
                }

                if space_empty.places < space_number.places {
                    space_number.places -= space_empty.places;
                    row[pointer_end] = TakenSpace::Number(space_number);
                    row.remove(pointer_start);
                    row.insert(
                        pointer_start,
                        TakenSpace::Number(Space {
                            id: space_number.id,
                            places: space_empty.places,
                        }),
                    );

                    pointer_start += 1;

                    continue;
                }
                panic!("Should be unreachable")
            }
            (TakenSpace::Number(_), _) => {
                pointer_start += 1;
                continue;
            }
            (_, TakenSpace::Empty(_)) => {
                pointer_end -= 1;
                continue;
            }
        }
    }
}

fn checksum(row: &Vec<TakenSpace>) -> i64 {
    let mut acc: i64 = 0;
    let mut counter_in_space = 0;

    for item in row {
        match item {
            TakenSpace::Empty(nnn) => counter_in_space += nnn.places,
            TakenSpace::Number(nnn) => {
                for _ in 0..nnn.places {
                    acc += (counter_in_space * nnn.id) as i64;
                    counter_in_space += 1;
                }
            }
        }
    }

    acc
}

#[allow(dead_code)]
fn row_viz(row: &Vec<TakenSpace>) {
    for item in row {
        match item {
            TakenSpace::Empty(nnn) => {
                for _ in 0..nnn.places {
                    print!(".",);
                }
            }
            TakenSpace::Number(nnn) => {
                for _ in 0..nnn.places {
                    print!("{}", nnn.id);
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut row = makerow(input);

    shift_row_p1(&mut row);

    Some(checksum(&row))
}

fn shift_row_p2(row: &mut Vec<TakenSpace>) {
    let mut pointer_end = row.len() - 1;

    while pointer_end > 0 {
        let mut pointer_start = 0;
        let mut moved = false;
        while pointer_end > pointer_start && !moved {
            let start_item = row[pointer_start];
            let end_item = row[pointer_end];

            match (start_item, end_item) {
                (TakenSpace::Empty(mut space_empty), TakenSpace::Number(space_number)) => {
                    if space_empty.places == space_number.places {
                        row[pointer_start] = end_item.clone();
                        row[pointer_end] = TakenSpace::Empty(Space {
                            id: 0,
                            places: space_number.places,
                        });

                        moved = true;

                        continue;
                    }

                    if space_empty.places > space_number.places {
                        space_empty.places -= space_number.places;
                        row[pointer_start] = TakenSpace::Empty(space_empty);
                        row[pointer_end] = TakenSpace::Empty(Space {
                            id: 0,
                            places: space_number.places,
                        });

                        row.insert(
                            pointer_start,
                            TakenSpace::Number(Space {
                                places: space_number.places,
                                id: space_number.id,
                            }),
                        );

                        moved = true;
                        continue;
                    }

                    if space_empty.places < space_number.places {
                        pointer_start += 1;
                        continue;
                    }
                    panic!("Should be unreachable")
                }
                (TakenSpace::Number(_), _) => {
                    pointer_start += 1;
                    continue;
                }
                (_, TakenSpace::Empty(_)) => {
                    pointer_end -= 1;
                    continue;
                }
            }
        }
        pointer_end -= 1
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut row = makerow(input);

    shift_row_p2(&mut row);

    Some(checksum(&row))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
