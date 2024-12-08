use crate::common;
use anyhow::Result;
use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/8.txt")?;

    let mut frequencies: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut size = 0;

    for (y, line) in lines.enumerate() {
        let line = line?;
        let line = line.trim();

        size = line.len() as i32;
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                match frequencies.entry(c) {
                    Entry::Occupied(mut e) => e.get_mut().push((x as i32, y as i32)),
                    Entry::Vacant(e) => {
                        e.insert(vec![(x as i32, y as i32)]);
                    }
                }
            }
        }
    }

    let mut antinodes_a = HashSet::new();
    let mut antinodes_b = HashSet::new();
    for antennae in frequencies.into_values() {
        for ((ax, ay), (bx, by)) in antennae.into_iter().tuple_combinations() {
            let dx = ax - bx;
            let dy = ay - by;
            let x1 = ax + dx;
            let y1 = ay + dy;
            let x2 = bx - dx;
            let y2 = by - dy;
            if 0 <= x1 && 0 <= y1 && x1 < size && y1 < size {
                antinodes_a.insert(x1 + y1 * size);
            }
            if 0 <= x2 && 0 <= y2 && x2 < size && y2 < size {
                antinodes_a.insert(x2 + y2 * size);
            }
            antinodes_b.insert(ax + ay * size);
            antinodes_b.insert(bx + by * size);
            let mut x = x1;
            let mut y = y1;
            while 0 <= x && 0 <= y && x < size && y < size {
                antinodes_b.insert(x + y * size);
                x += dx;
                y += dy;
            }
            let mut x = x2;
            let mut y = y2;
            while 0 <= x && 0 <= y && x < size && y < size {
                antinodes_b.insert(x + y * size);
                x -= dx;
                y -= dy;
            }
        }
    }

    let solution_a = antinodes_a.len();

    antinodes_b.extend(antinodes_a);

    let solution_b = antinodes_b.len();

    Ok((solution_a, solution_b))
}
