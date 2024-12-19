use crate::common;
use anyhow::Result;
use std::collections::HashMap;

pub fn main() -> Result<(usize, usize)> {
    let mut lines = common::read_lines("inputs/19.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let towels = lines
        .next()
        .unwrap()?
        .trim()
        .split(", ")
        .map(ToOwned::to_owned)
        .collect::<Vec<String>>();
    lines.next().unwrap()?;

    for line in lines {
        let line = line?;
        let line = line.trim();

        let count = is_possible(line, &towels, &mut HashMap::new());
        if count > 0 {
            solution_a += 1;
        }
        solution_b += count;
    }

    Ok((solution_a, solution_b))
}

fn is_possible(s: &str, towels: &[String], cache: &mut HashMap<String, usize>) -> usize {
    if s.is_empty() {
        return 1;
    }
    if cache.contains_key(s) {
        return cache[s];
    }
    let mut count = 0;
    for t in towels {
        if s.starts_with(t) {
            count += is_possible(&s[t.len()..], towels, cache);
        }
    }
    cache.insert(s.to_owned(), count);
    count
}
