use crate::common;
use anyhow::Result;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/20.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut grid = Vec::new();

    let mut start_x = 0;
    let mut start_y = 0;
    let mut end_x = 0;
    let mut end_y = 0;

    for (i, line) in lines.enumerate() {
        let line = line?;
        let line = line.trim();

        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start_x = j;
                    start_y = i;
                }
                'E' => {
                    end_x = j;
                    end_y = i;
                }
                _ => {}
            }
        }
        grid.push(line.chars().map(|c| c == '#').collect::<Vec<_>>());
    }

    let (distances_from_start, min_cost) = calc_distances(&grid, start_x, start_y, end_x, end_y);
    let (distances_from_end, _) = calc_distances(&grid, end_x, end_y, start_x, start_y);

    for (((sx, sy), distance_from_start), ((ex, ey), distance_from_end)) in distances_from_start
        .into_iter()
        .cartesian_product(&distances_from_end)
    {
        let distance = sx.abs_diff(*ex) + sy.abs_diff(*ey);
        let cost = distance_from_start + distance + distance_from_end;
        if cost < min_cost && min_cost - cost >= 100 {
            if distance <= 2 {
                solution_a += 1;
            } else if distance <= 20 {
                solution_b += 1;
            }
        }
    }

    solution_b += solution_a;

    Ok((solution_a, solution_b))
}

fn calc_distances(
    grid: &[Vec<bool>],
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
) -> (HashMap<(usize, usize), usize>, usize) {
    let size = grid.len();
    let mut min_cost = 0;
    let mut frontier = BinaryHeap::new();
    let mut g_score = HashMap::new();

    frontier.push(Reverse((0, start_x, start_y)));

    g_score.insert((start_x, start_y), 0);

    while let Some(Reverse((cost, x, y))) = frontier.pop() {
        if x == end_x && y == end_y {
            if min_cost == 0 {
                min_cost = cost;
            }
            continue;
        }

        if x < size - 1 && !grid[y][x + 1] {
            match g_score.entry((x + 1, y)) {
                Entry::Occupied(mut e) => {
                    if *e.get() > cost + 1 {
                        *e.get_mut() = cost + 1;
                        frontier.push(Reverse((cost + 1, x + 1, y)));
                    }
                }
                Entry::Vacant(e) => {
                    e.insert(cost + 1);
                    frontier.push(Reverse((cost + 1, x + 1, y)));
                }
            }
        }
        if y < size - 1 && !grid[y + 1][x] {
            match g_score.entry((x, y + 1)) {
                Entry::Occupied(mut e) => {
                    if *e.get() > cost + 1 {
                        *e.get_mut() = cost + 1;
                        frontier.push(Reverse((cost + 1, x, y + 1)));
                    }
                }
                Entry::Vacant(e) => {
                    e.insert(cost + 1);
                    frontier.push(Reverse((cost + 1, x, y + 1)));
                }
            }
        }
        if x > 0 && !grid[y][x - 1] {
            match g_score.entry((x - 1, y)) {
                Entry::Occupied(mut e) => {
                    if *e.get() > cost + 1 {
                        *e.get_mut() = cost + 1;
                        frontier.push(Reverse((cost + 1, x - 1, y)));
                    }
                }
                Entry::Vacant(e) => {
                    e.insert(cost + 1);
                    frontier.push(Reverse((cost + 1, x - 1, y)));
                }
            }
        }
        if y > 0 && !grid[y - 1][x] {
            match g_score.entry((x, y - 1)) {
                Entry::Occupied(mut e) => {
                    if *e.get() > cost + 1 {
                        *e.get_mut() = cost + 1;
                        frontier.push(Reverse((cost + 1, x, y - 1)));
                    }
                }
                Entry::Vacant(e) => {
                    e.insert(cost + 1);
                    frontier.push(Reverse((cost + 1, x, y - 1)));
                }
            }
        }
    }
    (g_score, min_cost)
}
