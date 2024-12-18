use crate::common;
use anyhow::Result;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
pub fn main() -> Result<(u32, String)> {
    let lines = common::read_lines("inputs/18.txt")?;

    let mut bytes = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        let (x, y) = line.split_once(',').unwrap();
        bytes.push((x.parse::<u32>()?, y.parse::<u32>()?));
    }

    let solution_a = calc_cost(&bytes.iter().take(1024).copied().collect()).unwrap();

    let mut min = 1024;
    let mut max = bytes.len() - 1;
    while min <= max {
        let mid = (min + max) / 2;
        if calc_cost(&bytes.iter().take(mid).copied().collect()).is_none() {
            max = mid - 1;
        } else {
            min = mid + 1;
        }
    }
    let solution_b = format!("{}, {}", bytes[min - 1].0, bytes[min - 1].1);

    Ok((solution_a, solution_b))
}

fn calc_cost(bytes: &HashSet<(u32, u32)>) -> Option<u32> {
    let size = 71;
    let goal_x = 70;
    let goal_y = 70;
    let mut frontier = BinaryHeap::new();
    let mut visited = HashSet::new();
    frontier.push(Reverse((0, 0, 0)));
    while let Some(Reverse((cost, x, y))) = frontier.pop() {
        if x == goal_x && y == goal_y {
            return Some(cost);
        }
        if !visited.insert((x, y)) {
            continue;
        }

        if x < size - 1 && !bytes.contains(&(x + 1, y)) {
            frontier.push(Reverse((cost + 1, x + 1, y)));
        }
        if y < size - 1 && !bytes.contains(&(x, y + 1)) {
            frontier.push(Reverse((cost + 1, x, y + 1)));
        }
        if x > 0 && !bytes.contains(&(x - 1, y)) {
            frontier.push(Reverse((cost + 1, x - 1, y)));
        }
        if y > 0 && !bytes.contains(&(x, y - 1)) {
            frontier.push(Reverse((cost + 1, x, y - 1)));
        }
    }

    None
}
