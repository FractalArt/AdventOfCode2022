//! # Advent of Code 2022 - Day 13
//!
//! This module contains the solution of the [thirteenth day's challenges](https://adventofcode.com/2022/day/13).
use serde_json::{
    from_str, json,
    Value::{self, Array, Number},
};

fn compare(l1: &Value, l2: &Value) -> bool {
    if let (Array(a), Array(b)) = (l1, l2) {
        for (x, y) in a.iter().zip(b) {
            match (x, y) {
                (Number(xx), Number(yy)) if xx == yy => continue,
                (Number(xx), Number(yy)) => return xx.as_u64().unwrap() < yy.as_u64().unwrap(),
                (Array(_), Number(yy)) => return compare(x, &json!([yy])),
                (Number(xx), Array(_)) => return compare(&json!([xx]), y),
                (Array(_), Array(_)) if x.as_array().unwrap() == y.as_array().unwrap() => continue,
                (Array(_), Array(_)) => return compare(x, y),
                _ => panic!("Only arrays and numbers possible"),
            }
        }
    }
    l1.as_array().unwrap().len() <= l2.as_array().unwrap().len()
}

/// The solution of part 1 from day 13.
pub fn day_13_1(data: &str) -> usize {
    data.split("\n\n")
        .map(|p| {
            let (p1, p2) = p.trim().split_once('\n').unwrap();
            let (p1, p2) = (
                from_str::<Value>(p1).unwrap(),
                from_str::<Value>(p2).unwrap(),
            );
            compare(&p1, &p2)
        })
        .enumerate()
        .filter_map(|(idx, cmp)| if cmp { Some(idx + 1) } else { None })
        .sum()
}

/// The solution of part 2 from day 13.
//pub fn day_13_2(data: &str) -> usize {
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_13_task_1() {
        let input = "[1,1,3,1,1]
            [1,1,5,1,1]

            [[1],[2,3,4]]
            [[1],4]

            [9]
            [[8,7,6]]

            [[4,4],4,4]
            [[4,4],4,4,4]

            [7,7,7,7]
            [7,7,7]

            []
            [3]

            [[[]]]
            [[]]

            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(day_13_1(&input), 13);
    }
}
