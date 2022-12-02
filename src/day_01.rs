//! # Advent of Code 2022 - Day 1
//!
//! This module contains the solution of the [first day's challenges](https://adventofcode.com/2022/day/1).
use itertools::Itertools;

/// The solution to task 1 of day 1.
pub fn day_01(data: &[String], take: usize) -> u32 {
    data.iter()
        .fold(vec![0], |mut state, x| {
            if x.is_empty() {
                state.push(0);
                state
            } else {
                *state.last_mut().unwrap() += x.parse::<u32>().unwrap();
                state
            }
        })
        .into_iter()
        .sorted()
        .rev()
        .take(take)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_01_1() {
        let input = vec![
            "1000".to_string(),
            "2000".to_string(),
            "3000".to_string(),
            "".to_string(),
            "4000".to_string(),
            "".to_string(),
            "5000".to_string(),
            "6000".to_string(),
            "".to_string(),
            "7000".to_string(),
            "8000".to_string(),
            "9000".to_string(),
            "".to_string(),
            "10000".to_string(),
        ];
        assert_eq!(day_01(&input, 1), 24000);
    }

    #[test]
    fn test_day_01_2() {
        let input = vec![
            "1000".to_string(),
            "2000".to_string(),
            "3000".to_string(),
            "".to_string(),
            "4000".to_string(),
            "".to_string(),
            "5000".to_string(),
            "6000".to_string(),
            "".to_string(),
            "7000".to_string(),
            "8000".to_string(),
            "9000".to_string(),
            "".to_string(),
            "10000".to_string(),
        ];
        assert_eq!(day_01(&input, 3), 45000);
    }
}
