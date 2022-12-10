//! # Advent of Code 2022 - Day 8
//!
//! This module contains the solution of the [eigth day's challenges](https://adventofcode.com/2022/day/8).
use itertools::Itertools;

/// The solution to task 1 of day 8.
pub fn day_08_1(data: &[String]) -> usize {
    let mut flags = vec![false; data.len().pow(2)];
    let matrix: Vec<Vec<u32>> = data
        .iter()
        .map(|row| row.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();

    for col in 0..data.len() {
        // rows, top to bottom
        let mut max_seen = 0;
        for row in 0..data.len() {
            if row == 0 || matrix[row][col] > max_seen {
                flags[row * data.len() + col] = true;
                max_seen = matrix[row][col];
            }
        }
        // rows, bottom to top
        max_seen = 0;
        for row in (0..data.len()).rev() {
            if row == data.len() - 1 || matrix[row][col] > max_seen {
                flags[row * data.len() + col] = true;
                max_seen = matrix[row][col];
            }
        }
    }

    for row in 0..data.len() {
        // cols, left to right
        let mut max_seen = 0;
        for col in 0..data.len() {
            if col == 0 || matrix[row][col] > max_seen {
                flags[row * data.len() + col] = true;
                max_seen = matrix[row][col];
            }
        }
        // cols, right to left
        max_seen = 0;
        for col in (1..data.len()).rev() {
            if col == data.len() - 1 || matrix[row][col] > max_seen {
                flags[row * data.len() + col] = true;
                max_seen = matrix[row][col];
            }
        }
    }

    flags.into_iter().filter(|&flag| flag).count()
}

/// The solution to task 2 of day 8.
pub fn day_08_2(data: &[String]) -> usize {
    let matrix: Vec<Vec<u32>> = data
        .iter()
        .map(|row| row.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();

    (1..matrix.len() - 1)
        .cartesian_product(1..matrix.len() - 1)
        .map(|(row, col)| {
            let height = matrix[row][col];
            let mut sum = 0;
            let mut prod = 1;
            for r in (0..row).rev() {
                sum += 1;
                if matrix[r][col] >= height {
                    break;
                }
            }

            prod *= sum;
            sum = 0;

            for r in matrix.iter().skip(row + 1) {
                sum += 1;
                if r[col] >= height {
                    break;
                }
            }

            prod *= sum;
            sum = 0;

            for c in (0..col).rev() {
                sum += 1;
                if matrix[row][c] >= height {
                    break;
                }
            }

            prod *= sum;
            sum = 0;

            for c in col + 1..matrix.len() {
                sum += 1;
                if matrix[row][c] >= height {
                    break;
                }
            }

            prod * sum
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_08_1() {
        let input = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];

        assert_eq!(day_08_1(&input), 21);
    }

    #[test]
    fn test_day_08_2() {
        let input = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];

        assert_eq!(day_08_2(&input), 8);
    }
}
