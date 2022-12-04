//! # Advent of Code 2022 - Day 4
//!
//! This module contains the solution of the [second day's challenges](https://adventofcode.com/2022/day/4).

/// The solution to task 1 of day 4.
pub fn day_04(data: &[String], partial_overlap: bool) -> usize {
    data.iter()
        .map(|line| line.split(',').collect::<Vec<_>>())
        .map(|split| {
            (
                split[0].split('-').collect::<Vec<_>>(),
                split[1].split('-').collect::<Vec<_>>(),
            )
        })
        .map(|(range_1, range_2)| {
            (
                (
                    range_1[0].parse::<i32>().unwrap(),
                    range_1[1].parse::<i32>().unwrap(),
                ),
                (
                    range_2[0].parse::<i32>().unwrap(),
                    range_2[1].parse::<i32>().unwrap(),
                ),
            )
        })
        .filter(|(range_1, range_2)| {
            if partial_overlap {
                (range_2.0 <= range_1.1 && range_2.0 >= range_1.0)
                    || (range_1.0 <= range_2.1 && range_1.0 >= range_2.0)
            } else {
                (range_1.0 >= range_2.0 && range_1.1 <= range_2.1)
                    || (range_2.0 >= range_1.0 && range_2.1 <= range_1.1)
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_04_1() {
        let input = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];
        assert_eq!(day_04(&input, false), 2);
    }

    #[test]
    fn test_day_04_2() {
        let input = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];
        assert_eq!(day_04(&input, true), 4);
    }
}
