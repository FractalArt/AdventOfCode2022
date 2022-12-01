//! # Advent of Code 2022 - Day 1
//!
//! This module contains the solution of the [first day's challenges](https://adventofcode.com/2021/day/1).

/// The solution to task 1 of day 1.
pub fn day_1(data: &[String]) -> u32 {
    data.iter()
        .fold((0, 0), |state, x| {
            if x.is_empty() {
                (std::cmp::max(state.0, state.1), 0)
            } else {
                (state.0, state.1 + x.parse::<u32>().unwrap())
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1_1() {
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
        assert_eq!(day_1(&input), 24000);
    }
}
