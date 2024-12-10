use crate::common;
use anyhow::Result;
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Space {
    Filled(usize, usize),
    Free(usize),
}

pub fn main() -> Result<(usize, usize)> {
    let mut disk_a = common::read_lines("inputs/9.txt")?
        .next()
        .unwrap()?
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let id = i / 2;
            let count = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                Space::Filled(id, count as usize)
            } else {
                Space::Free(count as usize)
            }
        })
        .collect::<Vec<_>>();
    let mut disk_b = disk_a.clone();
    let mut definitely_filled = 0;
    let mut i = disk_a.len() - 1;
    'outer: while i > 0 {
        match disk_a[i] {
            Space::Filled(id, count) => {
                let mut remaining = count;
                while remaining > 0 {
                    if definitely_filled >= i {
                        break 'outer;
                    }
                    if let Some((pos, Space::Free(space))) = disk_a[definitely_filled..i]
                        .iter()
                        .find_position(|space| matches!(space, Space::Free(_)))
                    {
                        let pos = definitely_filled + pos;
                        let space = *space;
                        match remaining.cmp(&space) {
                            Ordering::Less => {
                                disk_a[pos] = disk_a[i];
                                disk_a[i] = Space::Free(remaining);
                                disk_a.insert(pos + 1, Space::Free(space - remaining));
                                definitely_filled = pos + 1;
                                continue 'outer;
                            }
                            Ordering::Equal => {
                                disk_a.swap(pos, i);
                                remaining = 0;
                            }
                            Ordering::Greater => {
                                disk_a[pos] = Space::Filled(id, space);
                                remaining -= space;
                                disk_a[i] = Space::Filled(id, remaining);
                                definitely_filled = pos + 1;
                            }
                        }
                    } else {
                        break 'outer;
                    }
                }
            }
            Space::Free(_) => {}
        }
        i -= 1;
    }
    let solution_a = checksum(&disk_a);

    let mut i = disk_b.len() - 1;
    while i > 0 {
        match disk_b[i] {
            Space::Filled(_, count) => {
                if let Some((pos, Space::Free(space))) = disk_b[..i]
                    .iter()
                    .find_position(|space| matches!(space, Space::Free(cnt) if *cnt >= count))
                {
                    let space = *space;
                    disk_b.swap(pos, i);
                    if count != space {
                        disk_b[i] = Space::Free(count);
                        disk_b.insert(pos + 1, Space::Free(space - count));
                        continue;
                    }
                }
            }
            Space::Free(_) => {}
        }
        i -= 1;
    }

    let solution_b = checksum(&disk_b);

    Ok((solution_a, solution_b))
}

fn checksum(disk: &[Space]) -> usize {
    disk.iter()
        .fold((0, 0), |(sum, count), it| match it {
            Space::Filled(id, cnt) => (
                sum + (count..count + cnt).map(|i| i * id).sum::<usize>(),
                count + cnt,
            ),
            Space::Free(cnt) => (sum, count + cnt),
        })
        .0
}
