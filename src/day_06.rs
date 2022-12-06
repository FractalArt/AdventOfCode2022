//! # Advent of Code 2022 - Day 6
//!
//! This module contains the solution of the [second day's challenges](https://adventofcode.com/2022/day/6).
use itertools::Itertools;
use std::collections::HashSet;

/// The solution to task 1 of day 4.
pub fn day_06(data: &str) -> usize {
    data.chars()
        .tuple_windows::<(_, _, _, _)>()
        .enumerate()
        .find_map(|(index, tuple)| {
            if HashSet::from([tuple.0, tuple.1, tuple.2, tuple.3]).len() == 4 {
                Some(index)
            } else {
                None
            }
        })
        .unwrap()
        + 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_06_1() {
        assert_eq!(day_06("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(day_06("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(day_06("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(day_06("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(day_06("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
