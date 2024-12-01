use crate::common;
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/1.txt")?;

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();
        let (l, r) = line.split_once("   ").expect("Invalid format");
        left.push(l.parse::<usize>()?);
        right.push(r.parse::<usize>()?);
    }

    left.sort_unstable();
    right.sort_unstable();

    let solution_a = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (l, r)| acc + l.abs_diff(*r));

    let counts = right
        .into_iter()
        .dedup_with_count()
        .map(|(count, n)| (n, count))
        .collect::<HashMap<usize, usize>>();
    let solution_b = left.into_iter().fold(0, |acc, n| {
        acc + n * counts.get(&n).copied().unwrap_or_default()
    });

    Ok((solution_a, solution_b))
}
