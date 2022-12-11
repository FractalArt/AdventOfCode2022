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
) -> (isize, isize) {
    let new_pos = match std::cmp::max((hx - tx).abs(), (hy - ty).abs()) {
        2 => (
            tx + (hx - tx).checked_div((hx - tx).abs()).unwrap_or(0),
            ty + (hy - ty).checked_div((hy - ty).abs()).unwrap_or(0),
        ),
        _ => (tx, ty),
    };
    visited.insert(new_pos);
    new_pos
}

/// The solution to task 1 of day 9.
pub fn day_09_1(data: &[String]) -> usize {
    data.iter()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|split| (split[0], split[1].parse::<isize>().unwrap()))
        .fold(
            (HashSet::new(), (0, 0), (0, 0)),
            |(mut visited, (hx, hy), (tx, ty)), (dir, steps)| {
                let delta_h = match dir {
                    "L" => (-1, 0),
                    "R" => (1, 0),
                    "U" => (0, 1),
                    _ => (0, -1),
                };

                let new_pos =
                    (0..steps).fold(((hx, hy), (tx, ty)), |((h_x, h_y), (t_x, t_y)), _| {
                        (
                            (h_x + delta_h.0, h_y + delta_h.1),
                            tail_from_head(
                                h_x + delta_h.0,
                                h_y + delta_h.1,
                                t_x,
                                t_y,
                                &mut visited,
                            ),
                        )
                    });
                (visited, new_pos.0, new_pos.1)
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

        assert_eq!(day_09_1(&input), 13);
    }
}
