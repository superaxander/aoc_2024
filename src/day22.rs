use crate::common;
use anyhow::Result;
use rustc_hash::{FxHashMap, FxHashSet};

pub fn main() -> Result<(u32, u32)> {
    let lines = common::read_lines("inputs/22.txt")?;

    let mut solution_a = 0;

    let mut sequence_value_mapping = FxHashMap::default();
    for line in lines {
        let line = line?;
        let line = line.trim();

        let mut sequences = FxHashSet::default();

        let mut number: u32 = line.parse()?;
        let mut diffs = [0, 0, 0, 0];
        for i in 0..2000 {
            let old = number % 10;
            number = ((number << 6) ^ number) & 0xFF_FFFF;
            number = ((number >> 5) ^ number) & 0xFF_FFFF;
            number = ((number << 11) ^ number) & 0xFF_FFFF;

            let bananas = number % 10;
            diffs[i % 4] = bananas - old;
            if i > 3 {
                let sequence = ((diffs[i % 4] & 0xF) << 12)
                    | ((diffs[(i - 1) % 4] & 0xF) << 8)
                    | ((diffs[(i - 2) % 4] & 0xF) << 4)
                    | (diffs[(i - 3) % 4] & 0xF);
                if sequences.insert(sequence) {
                    *sequence_value_mapping.entry(sequence).or_insert(0) += bananas;
                }
            }
        }
        solution_a += number;
    }

    let solution_b = *sequence_value_mapping.values().max().unwrap();

    Ok((solution_a, solution_b))
}
