use crate::common;
use anyhow::Result;
use std::str::pattern::{Pattern, SearchStep, Searcher};

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/4.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut transposed = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();
        solution_a += count_matches("XMAS", line);
        solution_a += count_matches("SAMX", line);
        if transposed.is_empty() {
            transposed.extend(line.chars().map(|c| vec![c]));
        } else {
            for (c, dest) in line.chars().zip(transposed.iter_mut()) {
                dest.push(c)
            }
        }
    }

    for line in transposed.iter() {
        let line: String = line.iter().collect();
        solution_a += count_matches("XMAS", &line);
        solution_a += count_matches("SAMX", &line);
    }

    for i in 0..transposed.len() {
        let mut lrud = String::new();
        let mut lrdu = String::new();
        let mut rlud = String::new();
        let mut rldu = String::new();
        for j in i..transposed.len() {
            lrud.push(transposed[j - i][j]);
            lrdu.push(transposed[transposed.len() - j - 1][j - i]);
            if i != 0 {
                rlud.push(transposed[j][j - i]);
                rldu.push(transposed[transposed.len() - (j - i) - 1][j]);
            }
        }
        for line in [lrud, lrdu, rlud, rldu] {
            solution_a += count_matches("XMAS", &line);
            solution_a += count_matches("SAMX", &line);
        }
    }

    for i in 1..(transposed.len() - 1) {
        for j in 1..(transposed.len() - 1) {
            if transposed[i][j] == 'A'
                && ((transposed[i - 1][j - 1] == 'M' && transposed[i + 1][j + 1] == 'S')
                    || (transposed[i - 1][j - 1] == 'S' && transposed[i + 1][j + 1] == 'M'))
                && ((transposed[i + 1][j - 1] == 'M' && transposed[i - 1][j + 1] == 'S')
                    || (transposed[i + 1][j - 1] == 'S' && transposed[i - 1][j + 1] == 'M'))
            {
                solution_b += 1;
            }
        }
    }

    Ok((solution_a, solution_b))
}

fn count_matches(needle: &str, haystack: &str) -> usize {
    let mut searcher = needle.into_searcher(haystack);
    let mut count = 0;
    loop {
        match searcher.next() {
            SearchStep::Match(_, _) => count += 1,
            SearchStep::Reject(_, _) => {}
            SearchStep::Done => break,
        }
    }
    count
}
