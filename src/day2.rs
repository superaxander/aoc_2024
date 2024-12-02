use crate::common;
use anyhow::Result;
use std::cmp::Reverse;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/2.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    for line in lines {
        let line = line?;
        let line = line.trim();
        let levels = line
            .split(' ')
            .map(|s| s.parse::<usize>())
            .collect::<std::result::Result<Vec<_>, _>>()?;
        match (
            levels.is_sorted() || levels.is_sorted_by_key(Reverse),
            differences_in_range(levels.iter()),
        ) {
            (true, true) => {
                solution_a += 1;
                solution_b += 1;
            }
            (true, false) => {
                // Can remove any
                for i in 0..levels.len() {
                    if differences_in_range(without_idx(levels.iter(), i)) {
                        solution_b += 1;
                        break;
                    }
                }
            }
            (_, _) => {
                // Need to recheck after removing
                for i in 0..levels.len() {
                    let without = without_idx(levels.iter(), i).copied().collect::<Vec<_>>();
                    if (without.is_sorted() || without.is_sorted_by_key(Reverse))
                        && differences_in_range(without.iter())
                    {
                        solution_b += 1;
                        break;
                    }
                }
            }
        }
    }

    Ok((solution_a, solution_b))
}

fn without_idx<'a>(
    levels: impl Iterator<Item = &'a usize>,
    idx: usize,
) -> impl Iterator<Item = &'a usize> {
    levels
        .enumerate()
        .filter(move |(j, _)| *j != idx)
        .map(|(_, n)| n)
}

fn differences_in_range<'a>(levels: impl Iterator<Item = &'a usize>) -> bool {
    levels
        .map_windows(|[l, r]| l.abs_diff(**r))
        .all(|diff| (1..=3).contains(&diff))
}
