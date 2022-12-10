//! # Advent of Code 2022 - Day 3
//!
//! This module contains the solution of the [third day's challenges](https://adventofcode.com/2022/day/3).
use std::collections::HashSet;

/// The solution to task 1 of day 3.
pub fn day_03_1(data: &[String]) -> u32 {
    data.iter()
        .map(|line| {
            let (first, second) = line.trim().split_at(line.len() / 2);
            *first
                .chars()
                .collect::<HashSet<_>>()
                .intersection(&second.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>()[0]
        })
        .map(|error| {
            if error.is_ascii_lowercase() {
                error as u32 - 96
            } else {
                error as u32 - 38
            }
        })
        .sum()
}

/// The solution to task 2 of day 3.
pub fn day_03_2(data: &[String]) -> u32 {
    data.chunks(3)
        .map(|chunk| {
            chunk
                .iter()
                .map(|rucksack| rucksack.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>()
        })
        .map(|chunk| {
            chunk[0]
                .intersection(&chunk[1])
                .cloned()
                .collect::<HashSet<_>>()
                .intersection(&chunk[2])
                .cloned()
                .last()
                .unwrap()
        })
        .map(|error| {
            if error.is_ascii_lowercase() {
                error as u32 - 96
            } else {
                error as u32 - 38
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_03_1() {
        let input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
        assert_eq!(day_03_1(&input), 157);
    }

    #[test]
    fn test_day_03_2() {
        let input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
        assert_eq!(day_03_2(&input), 70);
    }
}
