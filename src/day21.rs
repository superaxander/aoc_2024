use crate::common;
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

const DOOR_KEYPAD_MAPPING: [(usize, usize); 18] = [
    (1, 0),
    (0, 1),
    (1, 1),
    (2, 1),
    (0, 2),
    (1, 2),
    (2, 2),
    (0, 3),
    (1, 3),
    (2, 3),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (2, 0),
];

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/21.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut cache = HashMap::new();
    for line in lines {
        let line = line?;
        let line = line.trim();

        let mut last_position = DOOR_KEYPAD_MAPPING['A' as usize - '0' as usize];
        let (sequence_2, sequence_25) = line
            .chars()
            .map(|c| {
                let target_position = DOOR_KEYPAD_MAPPING[c as usize - '0' as usize];
                let mut min_2 = usize::MAX;
                let mut min_25 = usize::MAX;
                for permutation in get_permutations(last_position, target_position, (0, 0)) {
                    let permutation = [&permutation[..], &['A']].concat();
                    min_2 = min_2.min(shortest_sequences(&permutation, 2, &mut cache));
                    min_25 = min_25.min(shortest_sequences(&permutation, 25, &mut cache));
                }
                last_position = target_position;
                (min_2, min_25)
            })
            .reduce(|(l2, l25), (r2, r25)| (l2 + r2, l25 + r25))
            .unwrap();
        let factor = line[..line.len() - 1].parse::<usize>()?;
        solution_a += sequence_2 * factor;
        solution_b += sequence_25 * factor;
    }

    Ok((solution_a, solution_b))
}

fn shortest_sequences(
    sequence: &[char],
    depth: usize,
    cache: &mut HashMap<(Vec<char>, usize), usize>,
) -> usize {
    if depth == 0 {
        return sequence.len();
    }
    if let Some(length) = cache.get(&(sequence.to_vec(), depth)) {
        return *length;
    }
    let mut sequence_length = 0;
    let mut last_position = (2, 1);
    for c in sequence {
        let target_position = match c {
            'A' => (2, 1),
            '^' => (1, 1),
            '<' => (0, 0),
            'v' => (1, 0),
            '>' => (2, 0),
            _ => panic!("Unexpected character {c}"),
        };

        let mut shortest = usize::MAX;
        for permutation in get_permutations(last_position, target_position, (0, 1)) {
            let length = shortest_sequences(&[&permutation[..], &['A']].concat(), depth - 1, cache);
            if length < shortest {
                shortest = length;
            }
        }
        sequence_length += shortest;

        last_position = target_position;
    }

    cache.insert((sequence.to_vec(), depth), sequence_length);

    sequence_length
}

fn get_permutations(
    last_position: (usize, usize),
    target_position: (usize, usize),
    dangerous_position: (usize, usize),
) -> impl Iterator<Item = Vec<char>> {
    let mut sequence = Vec::new();
    if target_position.0 < last_position.0 {
        sequence.extend(['<'].repeat(last_position.0 - target_position.0));
    } else {
        sequence.extend(['>'].repeat(target_position.0 - last_position.0));
    }
    if target_position.1 < last_position.1 {
        sequence.extend(['v'].repeat(last_position.1 - target_position.1));
    } else {
        sequence.extend(['^'].repeat(target_position.1 - last_position.1));
    }
    let length = sequence.len();
    sequence
        .into_iter()
        .permutations(length)
        .filter(move |seq| valid_permutation(seq, last_position, dangerous_position))
}

fn valid_permutation(
    sequence: &[char],
    last_position: (usize, usize),
    dangerous_position: (usize, usize),
) -> bool {
    let mut position = last_position;
    let mut found = false;
    for c in sequence {
        match c {
            '<' => position.0 -= 1,
            '>' => position.0 += 1,
            'v' => position.1 -= 1,
            '^' => position.1 += 1,
            _ => panic!("Unexpected character {c}"),
        }
        if position == dangerous_position {
            found = true;
            break;
        }
    }
    !found
}
