use crate::common;
use crate::common::Facing;
use anyhow::Result;
use std::cmp::{Ordering, Reverse};
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet};

type State = (usize, usize, Facing);

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/16.txt")?;

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

    let mut solution_a = 0;
    let mut frontier = BinaryHeap::new();
    let mut g_score = HashMap::new();
    let mut remaining = Vec::new();

    frontier.push(Reverse((0, (start_x, start_y, Facing::East))));

    while let Some(Reverse((score, current @ (x, y, facing)))) = frontier.pop() {
        if x == end_x && y == end_y {
            if solution_a == 0 {
                solution_a = score;
            } else if score > solution_a {
                break;
            }
            if !remaining.contains(&current) {
                remaining.push(current);
            }
        }

        match facing {
            Facing::North if !grid[y - 1][x] => add_frontier(
                &mut frontier,
                &mut g_score,
                score + 1,
                (x, y - 1, facing),
                current,
            ),
            Facing::East if !grid[y][x + 1] => add_frontier(
                &mut frontier,
                &mut g_score,
                score + 1,
                (x + 1, y, facing),
                current,
            ),
            Facing::South if !grid[y + 1][x] => add_frontier(
                &mut frontier,
                &mut g_score,
                score + 1,
                (x, y + 1, facing),
                current,
            ),
            Facing::West if !grid[y][x - 1] => add_frontier(
                &mut frontier,
                &mut g_score,
                score + 1,
                (x - 1, y, facing),
                current,
            ),
            _ => {}
        }
        add_frontier(
            &mut frontier,
            &mut g_score,
            score + 1000,
            (x, y, facing.left()),
            current,
        );
        add_frontier(
            &mut frontier,
            &mut g_score,
            score + 1000,
            (x, y, facing.right()),
            current,
        );
    }

    let mut good_spots = HashSet::new();
    while let Some((x, y, facing)) = remaining.pop() {
        good_spots.insert((x, y));
        if x == start_x && y == start_y {
            continue;
        }

        remaining.extend(&g_score[&(x, y, facing)].1);
    }

    Ok((solution_a, good_spots.len()))
}

fn add_frontier(
    frontier: &mut BinaryHeap<Reverse<(usize, State)>>,
    g_score: &mut HashMap<State, (usize, Vec<State>)>,
    score: usize,
    new: State,
    current: State,
) {
    match g_score.entry(new) {
        Entry::Occupied(mut e) => {
            let (old, predecessors) = e.get_mut();
            match score.cmp(old) {
                Ordering::Less => {
                    *old = score;
                    predecessors.clear();
                    predecessors.push(current);
                }
                Ordering::Equal => {
                    predecessors.push(current);
                }
                Ordering::Greater => {}
            }
        }
        Entry::Vacant(e) => {
            e.insert((score, vec![current]));
            frontier.push(Reverse((score, new)));
        }
    }
}
