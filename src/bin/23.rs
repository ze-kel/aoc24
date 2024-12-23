use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

fn build_computers(input: &str) -> HashMap<String, HashSet<String>> {
    let mut computers: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.lines() {
        let (left, right) = line.split_once("-").unwrap();

        let ll = computers.entry(left.to_string()).or_insert(HashSet::new());
        ll.insert(right.to_string());

        let rr = computers.entry(right.to_string()).or_insert(HashSet::new());
        rr.insert(left.to_string());
    }
    computers
}

pub fn part_one(input: &str) -> Option<u32> {
    let computers = build_computers(input);
    let mut triplets: HashSet<String> = HashSet::new();

    for comp in computers.keys() {
        if !comp.starts_with('t') {
            continue;
        }

        let my_connections = computers.get(comp).unwrap();

        for conn1 in my_connections.iter() {
            if conn1 == comp {
                continue;
            }

            let c1c = computers.get(conn1).unwrap();

            for conn2 in my_connections.iter() {
                if conn2 == comp || conn2 == conn1 {
                    continue;
                }

                if c1c.contains(conn2) {
                    let mut connvec: Vec<String> = vec![comp.clone(), conn1.clone(), conn2.clone()];
                    connvec.sort();

                    triplets.insert(connvec.join(","));
                }
            }
        }
    }

    Some(triplets.len() as u32)
}

fn bron_kerbosch(
    current: &mut HashSet<String>,
    candidates: HashSet<String>,
    excluded: HashSet<String>,
    graph: &HashMap<String, HashSet<String>>,
    max_clique: &mut HashSet<String>,
) {
    if candidates.is_empty() && excluded.is_empty() {
        if current.len() > max_clique.len() {
            max_clique.clear();
            max_clique.extend(current.iter().cloned());
        }
        return;
    }

    let mut c_c = candidates.clone();
    for v in candidates {
        let conns = match graph.get(&v) {
            Some(v) => v,
            None => &HashSet::new(),
        };

        current.insert(v.clone());

        let n_cand: HashSet<_> = c_c.intersection(conns).cloned().collect();
        let n_ex: HashSet<_> = excluded.intersection(conns).cloned().collect();

        bron_kerbosch(current, n_cand, n_ex, graph, max_clique);

        current.remove(&v);
        c_c.remove(&v);
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let computers = build_computers(input);

    let mut max_clique: HashSet<String> = HashSet::new();

    bron_kerbosch(
        &mut HashSet::new(),
        computers.keys().cloned().collect(),
        HashSet::new(),
        &computers,
        &mut max_clique,
    );

    let mut vectorized: Vec<String> = max_clique.iter().map(|v| v.clone()).collect();
    vectorized.sort();

    Some(vectorized.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
