//! # Advent of Code 2022 - Day 9
//!
//! This module contains the solution of the [ninth day's challenges](https://adventofcode.com/2022/day/9).
use std::collections::HashSet;

fn tail_from_head(
    hx: isize,
    hy: isize,
    tx: isize,
    ty: isize,
    visited: &mut HashSet<(isize, isize)>,
    tail: bool,
) -> (isize, isize) {
    let new_pos = match std::cmp::max((hx - tx).abs(), (hy - ty).abs()) {
        2 => (
            tx + (hx - tx).checked_div((hx - tx).abs()).unwrap_or(0),
            ty + (hy - ty).checked_div((hy - ty).abs()).unwrap_or(0),
        ),
        _ => (tx, ty),
    };
    if tail {
        visited.insert(new_pos);
    }
    new_pos
}

/// The solution of day 9.
pub fn day_09(data: &[String], knots: usize) -> usize {
    data.iter()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|split| (split[0], split[1].parse::<isize>().unwrap()))
        .fold(
            (HashSet::new(), vec![(0, 0); knots]),
            |(mut visited, mut curr), (dir, steps)| {
                let delta_h = match dir {
                    "L" => (-1, 0),
                    "R" => (1, 0),
                    "U" => (0, 1),
                    _ => (0, -1),
                };

                (0..steps).for_each(|_| {
                    curr[0] = (curr[0].0 + delta_h.0, curr[0].1 + delta_h.1);
                    for i in 1..curr.len() {
                        curr[i] = tail_from_head(
                            curr[i - 1].0,
                            curr[i - 1].1,
                            curr[i].0,
                            curr[i].1,
                            &mut visited,
                            i + 1 == knots,
                        );
                    }
                });

                (visited, curr)
            },
        )
        .0
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_09_1() {
        let input = vec![
            "R 4".to_string(),
            "U 4".to_string(),
            "L 3".to_string(),
            "D 1".to_string(),
            "R 4".to_string(),
            "D 1".to_string(),
            "L 5".to_string(),
            "R 2".to_string(),
        ];

        assert_eq!(day_09(&input, 2), 13);
    }

    #[test]
    fn test_day_09_2() {
        let input = vec![
            "R 4".to_string(),
            "U 4".to_string(),
            "L 3".to_string(),
            "D 1".to_string(),
            "R 4".to_string(),
            "D 1".to_string(),
            "L 5".to_string(),
            "R 2".to_string(),
        ];

        assert_eq!(day_09(&input, 10), 1);

        let input = vec![
            "R 5".to_string(),
            "U 8".to_string(),
            "L 8".to_string(),
            "D 3".to_string(),
            "R 17".to_string(),
            "D 10".to_string(),
            "L 25".to_string(),
            "U 20".to_string(),
        ];

        assert_eq!(day_09(&input, 10), 36);
    }
}
