use crate::common;
use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::once;

pub fn main() -> Result<(usize, String)> {
    let lines = common::read_lines("inputs/23.txt")?;

    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        let (l, r) = line.split_once('-').unwrap();
        connections
            .entry(l.to_owned())
            .or_default()
            .insert(r.to_owned());
        connections
            .entry(r.to_owned())
            .or_default()
            .insert(l.to_owned());
    }

    let mut sets = HashSet::new();
    for k in connections.keys().filter(|k| k.starts_with('t')) {
        for (k2, k3) in connections[k].iter().tuple_combinations() {
            if connections[k2].contains(k3) {
                let mut set = [k.clone(), k2.clone(), k3.clone()];
                set.sort();
                sets.insert(set);
            }
        }
    }

    let solution_a = sets.len();

    let mut min_sets: HashMap<usize, Vec<String>> = HashMap::new();
    let mut max_set = None;
    for (k, v) in &connections {
        for i in 1..v.len() {
            min_sets.entry(i).or_default().push(k.clone());
        }
    }

    // Iterating seems to actually be unnecessary here
    'outer: for i in (0..=*min_sets.keys().max().unwrap()).rev() {
        for k in &min_sets[&i] {
            for others in connections[k].iter().combinations(i) {
                if others
                    .iter()
                    .cartesian_product(others.iter())
                    .all(|(k2, k3)| k2 == k3 || connections[*k2].contains(*k3))
                {
                    max_set = Some(
                        others
                            .into_iter()
                            .cloned()
                            .chain(once(k.to_owned()))
                            .collect::<Vec<String>>(),
                    );
                    break 'outer;
                }
            }
        }
    }

    let mut max_set = max_set.unwrap();
    max_set.sort_unstable();

    let solution_b = max_set.join(",");

    Ok((solution_a, solution_b))
}
