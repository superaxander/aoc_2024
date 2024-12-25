use crate::common;
use anyhow::Result;
use itertools::Itertools;

pub fn main() -> Result<(usize, &'static str)> {
    const HEIGHT: usize = 7;
    const WIDTH: usize = 5;

    let lines = common::read_lines("inputs/25.txt")?;

    let mut solution_a = 0;

    let mut locks = Vec::new();
    let mut keys = Vec::new();
    let mut is_lock = false;

    let mut heights = [0; WIDTH];

    for (i, line) in lines.enumerate() {
        let line = line?;
        let line = line.trim();

        match i % (HEIGHT + 1) {
            0 => {
                is_lock = line == "#####";
                if !is_lock {
                    line.chars().enumerate().for_each(|(k, c)| {
                        if c == '#' {
                            heights[k] = HEIGHT - 1;
                        }
                    });
                }
            }
            HEIGHT => {
                if is_lock {
                    locks.push(heights);
                } else {
                    keys.push(heights);
                }
                heights = [0; WIDTH];
            }
            j if is_lock => line.chars().enumerate().for_each(|(k, c)| {
                if c == '#' {
                    heights[k] = heights[k].max(j);
                }
            }),
            j => line.chars().enumerate().for_each(|(k, c)| {
                if c == '#' {
                    heights[k] = heights[k].max(HEIGHT - j - 1);
                }
            }),
        }
    }
    if is_lock {
        locks.push(heights);
    } else {
        keys.push(heights);
    }

    for (k, l) in locks.iter().cartesian_product(keys.iter()) {
        if k.iter().zip(l.iter()).all(|(k, l)| k + l < HEIGHT - 1) {
            solution_a += 1;
        }
    }

    Ok((solution_a, "Merry Christmas"))
}
