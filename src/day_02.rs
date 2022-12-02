//! # Advent of Code 2022 - Day 2
//!
//! This module contains the solution of the [second day's challenges](https://adventofcode.com/2022/day/2).

/// The solution to task 1 of day 2.
pub fn day_02_1(data: &[String]) -> u32 {
    data.iter()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|letters| match (letters[0], letters[1]) {
            ("A", "X") => 4,
            ("A", "Y") => 8,
            ("A", "Z") => 3,
            ("B", "X") => 1,
            ("B", "Y") => 5,
            ("B", "Z") => 9,
            ("C", "X") => 7,
            ("C", "Y") => 2,
            _ => 6,
        })
        .sum()
}

/// The solution to task 2 of day 2.
pub fn day_02_2(data: &[String]) -> u32 {
    data.iter()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|letters| match (letters[0], letters[1]) {
            ("A", "X") => 3,
            ("A", "Y") => 4,
            ("A", "Z") => 8,
            ("B", "X") => 1,
            ("B", "Y") => 5,
            ("B", "Z") => 9,
            ("C", "X") => 2,
            ("C", "Y") => 6,
            _ => 7,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_02_1() {
        let input = vec!["A Y".to_string(), "B X".to_string(), "C Z".to_string()];
        assert_eq!(day_02_1(&input), 15);
    }

    #[test]
    fn test_day_02_2() {
        let input = vec!["A Y".to_string(), "B X".to_string(), "C Z".to_string()];
        assert_eq!(day_02_2(&input), 12);
    }
}
