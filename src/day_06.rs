//! # Advent of Code 2022 - Day 6
//!
//! This module contains the solution of the [second day's challenges](https://adventofcode.com/2022/day/6).
use std::collections::HashSet;

/// The solution to task 1 of day 6.
pub fn day_06(data: &str, take: usize) -> usize {
    data.as_bytes()
        .windows(take)
        .enumerate()
        .map(|(index, window)| (index, window.iter().collect::<HashSet<_>>()))
        .find_map(|(index, set)| {
            if set.len() == take {
                Some(index + take)
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_06_1() {
        assert_eq!(day_06("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(day_06("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(day_06("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(day_06("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(day_06("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test_day_06_2() {
        assert_eq!(day_06("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(day_06("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(day_06("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(day_06("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(day_06("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
