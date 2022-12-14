//! # Advent of Code 2022 - Day 5
//!
//! This module contains the solution of the [fifth day's challenges](https://adventofcode.com/2022/day/5).
use regex::Regex;
use std::collections::LinkedList;

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"move (\d*) from (\d*) to (\d*)$").unwrap();
}

type Stacks = Vec<LinkedList<char>>;
type Rules = Vec<(usize, usize, usize)>;

fn parser(input: &[String]) -> (Stacks, Rules) {
    let mut split = input.splitn(2, |line| line.trim() == "");

    let stack: Vec<_> = split.next().unwrap().iter().collect();
    let n_stacks = stack[stack.len() - 1]
        .split(' ')
        .filter(|&s| !s.is_empty())
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let mut stacks = vec![LinkedList::<char>::new(); n_stacks];

    stack[..stack.len() - 1].iter().for_each(|line| {
        line.as_bytes()
            .chunks(4)
            .enumerate()
            .for_each(|(index, chunk)| {
                if chunk[1] != b' ' {
                    stacks[index].push_front(chunk[1] as char)
                }
            })
    });

    let rules = split
        .next()
        .unwrap()
        .iter()
        .map(|line| {
            let captures = RE.captures(line).unwrap();
            (
                // Change indices from 1-based to 0-based for the `from` and `to` indices
                captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
                captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
            )
        })
        .collect();

    (stacks, rules)
}

/// The solution to task 1 of day 5.
pub fn day_05_1(data: &[String]) -> String {
    let (mut stacks, rules) = parser(data);
    rules.into_iter().for_each(|(n, from, to)| {
        (0..n).for_each(|_| {
            let x = stacks[from].pop_back().unwrap();
            stacks[to].push_back(x);
        });
    });

    stacks
        .into_iter()
        .map(|mut stack| stack.pop_back().unwrap_or(' '))
        .collect::<String>()
}

/// The solution to task 2 of day 5.
pub fn day_05_2(data: &[String]) -> String {
    let (mut stacks, rules) = parser(data);
    rules.into_iter().for_each(|(n, from, to)| {
        let split = stacks[from].len() - n;
        let mut to_append = stacks[from].split_off(split);
        stacks[to].append(&mut to_append);
    });

    stacks
        .into_iter()
        .map(|mut stack| stack.pop_back().unwrap_or(' '))
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_05_1() {
        let input = vec![
            "    [D]    ".to_string(),
            "[N] [C]    ".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3 ".to_string(),
            "".to_string(),
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
            "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string(),
        ];

        assert_eq!(day_05_1(&input), "CMZ");
    }

    #[test]
    fn test_day_05_2() {
        let input = vec![
            "    [D]    ".to_string(),
            "[N] [C]    ".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3 ".to_string(),
            "".to_string(),
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
            "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string(),
        ];

        assert_eq!(day_05_2(&input), "MCD");
    }
}
