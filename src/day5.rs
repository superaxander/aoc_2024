use crate::common;
use anyhow::Result;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/5.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut orderings = Vec::new();
    let mut do_updates = false;

    for line in lines {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            do_updates = true;
            orderings.sort_unstable();
        } else if do_updates {
            let mut update = line
                .split(',')
                .map(|s| s.parse::<usize>())
                .collect::<std::result::Result<Vec<_>, _>>()?;
            if sort(&orderings, &mut update) {
                solution_b += update[(update.len()) / 2];
            } else {
                solution_a += update[(update.len()) / 2];
            }
        } else {
            let (before, after) = line.split_once("|").unwrap();
            orderings.push((before.parse::<usize>()?, after.parse::<usize>()?));
        }
    }

    Ok((solution_a, solution_b))
}

fn sort(orderings: &[(usize, usize)], update: &mut Vec<usize>) -> bool {
    let mut changed = false;
    loop {
        let mut swapped = false;

        for i in 1..update.len() {
            if let Some(start) = orderings
                .iter()
                .position(|(before, _)| *before == update[i])
            {
                let until = orderings[start..].partition_point(|(before, _)| *before == update[i]);
                for j in 0..i {
                    if orderings[start..start + until]
                        .iter()
                        .any(|(_, after)| *after == update[j])
                    {
                        update.swap(i, j);
                        swapped = true;
                        changed = true;
                    }
                }
            }
        }

        if !swapped {
            return changed;
        }
    }
}
