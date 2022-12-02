//! # Advent of Code 2022 - Day 2
//!
//! This module contains the solution of the [second day's challenges](https://adventofcode.com/2022/day/2).

/// The solution to task 1 of day 2.
pub fn day_02(data: &[String]) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_02_1() {
        let input = vec!["A Y".to_string(), "B X".to_string(), "C Z".to_string()];
        assert_eq!(day_02(&input), 15);
    }
}
