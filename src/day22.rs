use crate::common;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/22.txt")?;

    let mut solution_a = 0;

    let mut sequences = HashSet::new();
    let mut mappings = Vec::new();
    for line in lines {
        let line = line?;
        let line = line.trim();

        let mut sequence_value_mapping = HashMap::new();

        let mut number: usize = line.parse()?;
        let mut diffs = [0, 0, 0, 0];
        for i in 0..2000 {
            let old = number % 10;
            number = ((number << 6) ^ number) & 0xFF_FFFF;
            number = ((number >> 5) ^ number) & 0xFF_FFFF;
            number = ((number << 11) ^ number) & 0xFF_FFFF;

            let bananas = number % 10;
            diffs[i % 4] = bananas - old;
            if i > 3 {
                let sequence = (
                    diffs[i % 4],
                    diffs[(i - 1) % 4],
                    diffs[(i - 2) % 4],
                    diffs[(i - 3) % 4],
                );
                sequence_value_mapping.entry(sequence).or_insert(bananas);
            }
        }
        sequences.extend(sequence_value_mapping.keys());
        mappings.push(sequence_value_mapping);
        solution_a += number;
    }

    let mut solution_b = 0;
    for sequence in sequences {
        let mut count = 0;
        for mapping in &mappings {
            count += mapping.get(&sequence).unwrap_or(&0);
        }
        solution_b = solution_b.max(count);
    }

    Ok((solution_a, solution_b))
}
