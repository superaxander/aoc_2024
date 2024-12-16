use crate::common;
use crate::common::Facing;
use anyhow::Result;
use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/16.txt")?;

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

        for (j, c) in line
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == 'E' || *c == 'S')
            .collect::<Vec<_>>()
        {
            match c {
                'S' => {
                    start_x = j;
                    start_y = i;
                }
                'E' => {
                    end_x = j;
                    end_y = i;
                }
                _ => unreachable!(),
            }
        }
        grid.push(line.chars().map(|c| c == '#').collect::<Vec<_>>());
    }

    let mut frontier = BinaryHeap::new();
    let mut g_score = HashMap::new();

    frontier.push(Reverse((0, start_x, start_y, Facing::East)));

    let mut good_spots = HashSet::new();
    good_spots.insert((end_x, end_y));

    while let Some(Reverse((score, x, y, facing))) = frontier.pop() {
        if x == end_x && y == end_y {
            if solution_a == 0 {
                solution_a = score;
            }
            if score > solution_a {
                break;
            }
        }

        match facing {
            Facing::North => {
                if !grid[y - 1][x] {
                    add_frontier(&mut frontier, &mut g_score, score + 1, x, y - 1, facing);
                }
            }
            Facing::East => {
                if !grid[y][x + 1] {
                    add_frontier(&mut frontier, &mut g_score, score + 1, x + 1, y, facing);
                }
            }
            Facing::South => {
                if !grid[y + 1][x] {
                    add_frontier(&mut frontier, &mut g_score, score + 1, x, y + 1, facing);
                }
            }
            Facing::West => {
                if !grid[y][x - 1] {
                    add_frontier(&mut frontier, &mut g_score, score + 1, x - 1, y, facing);
                }
            }
        }
        add_frontier(
            &mut frontier,
            &mut g_score,
            score + 1000,
            x,
            y,
            facing.left(),
        );
        add_frontier(
            &mut frontier,
            &mut g_score,
            score + 1000,
            x,
            y,
            facing.right(),
        );
    }

    let mut frontier = BinaryHeap::new();
    frontier.push(Reverse((0, start_x, start_y, Facing::East, vec![])));

    while let Some(Reverse((score, x, y, facing, history))) = frontier.pop() {
        if score > solution_a {
            break;
        }
        if x == end_x && y == end_y {
            good_spots.extend(history.into_iter().map(|(x, y, _)| (x, y)));
            continue;
        }
        if g_score
            .get(&(x, y, facing))
            .is_some_and(|s| *s < score)
        {
            continue;
        }
        if history.contains(&(x, y, facing)) {
            continue;
        }

        match facing {
            Facing::North => {
                if !grid[y - 1][x] {
                    let mut history = history.clone();
                    history.push((x, y, facing));
                    frontier.push(Reverse((score + 1, x, y - 1, facing, history)));
                }
            }
            Facing::East => {
                if !grid[y][x + 1] {
                    let mut history = history.clone();
                    history.push((x, y, facing));
                    frontier.push(Reverse((score + 1, x + 1, y, facing, history)));
                }
            }
            Facing::South => {
                if !grid[y + 1][x] {
                    let mut history = history.clone();
                    history.push((x, y, facing));
                    frontier.push(Reverse((score + 1, x, y + 1, facing, history)));
                }
            }
            Facing::West => {
                if !grid[y][x - 1] {
                    let mut history = history.clone();
                    history.push((x, y, facing));
                    frontier.push(Reverse((score + 1, x - 1, y, facing, history)));
                }
            }
        }

        frontier.push(Reverse((
            score + 1000,
            x,
            y,
            facing.left(),
            history.clone(),
        )));
        frontier.push(Reverse((score + 1000, x, y, facing.right(), history)));
    }

    let solution_b = good_spots.len();
    Ok((solution_a, solution_b))
}

fn add_frontier(
    frontier: &mut BinaryHeap<Reverse<(usize, usize, usize, Facing)>>,
    g_score: &mut HashMap<(usize, usize, Facing), usize>,
    score: usize,
    x: usize,
    y: usize,
    facing: Facing,
) {
    match g_score.entry((x, y, facing)) {
        Entry::Occupied(mut e) => {
            let old = e.get_mut();
            if *old > score {
                *old = score;
            }
        }
        Entry::Vacant(mut e) => {
            e.insert(score);
            frontier.push(Reverse((score, x, y, facing)));
        }
    }
}
