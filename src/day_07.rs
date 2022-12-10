//! # Advent of Code 2022 - Day 7
//!
//! This module contains the solution of the [seventh day's challenges](https://adventofcode.com/2022/day/7).
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;

fn hash(v: &[&str], remain: &[&str]) -> u64 {
    let mut hasher = DefaultHasher::new();
    v.iter()
        .chain(remain.iter())
        .for_each(|v| v.hash(&mut hasher));
    hasher.finish()
}

fn compute_dir_sizes(data: &[String]) -> HashMap<u64, usize> {
    let mut parents = Vec::<&str>::new();
    let mut dir_sizes = HashMap::<u64, usize>::new();
    let mut treated_files = HashSet::<u64>::new();
    let mut current = "/";

    data.iter().filter(|&line| line != "$ ls").for_each(|line| {
        let split: Vec<_> = line.split(' ').collect();
        match (split[0], split[1]) {
            ("$", "cd") => match split[2] {
                "/" => {
                    current = "/";
                    parents = Default::default();
                    dir_sizes.entry(hash(&["/"], &[])).or_insert(0);
                }
                ".." => {
                    current = parents.pop().unwrap();
                }
                d => {
                    parents.push(current);
                    current = d;
                }
            },
            ("dir", d) => {
                dir_sizes.entry(hash(&parents, &[d])).or_insert(0);
            }
            (size, file) => {
                let size = size.parse::<usize>().unwrap();
                let file_hash = hash(&parents, &[current, file]);
                if !treated_files.contains(&file_hash) {
                    treated_files.insert(file_hash);
                    *dir_sizes.entry(hash(&parents, &[current])).or_insert(0) += size;

                    parents.iter().enumerate().for_each(|(idx, _)| {
                        *dir_sizes.entry(hash(&parents[..idx + 1], &[])).or_insert(0) += size;
                    })
                }
            }
        }
    });
    dir_sizes
}

/// The solution to task 1 of day 7.
pub fn day_07_1(data: &[String]) -> usize {
    compute_dir_sizes(data)
        .values()
        .filter(|&&val| val <= 100000)
        .sum()
}

/// The solution to task 2 of day 7.
pub fn day_07_2(data: &[String]) -> usize {
    let dir_sizes = compute_dir_sizes(data);
    let used = dir_sizes.values().max().unwrap();
    dir_sizes
        .values()
        .filter_map(|&val| (70000000 - used + val >= 30000000).then_some(val))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_07_1() {
        let input = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
            "$ ls".to_string(),
            "dir e".to_string(),
            "29116 f".to_string(),
            "2557 g".to_string(),
            "62596 h.lst".to_string(),
            "$ cd e".to_string(),
            "$ ls".to_string(),
            "584 i".to_string(),
            "$ cd ..".to_string(),
            "$ cd ..".to_string(),
            "$ cd d".to_string(),
            "$ ls".to_string(),
            "4060174 j".to_string(),
            "8033020 d.log".to_string(),
            "5626152 d.ext".to_string(),
            "7214296 k".to_string(),
        ];

        assert_eq!(day_07_1(&input), 95437);
    }

    #[test]
    fn test_day_07_2() {
        let input = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
            "$ ls".to_string(),
            "dir e".to_string(),
            "29116 f".to_string(),
            "2557 g".to_string(),
            "62596 h.lst".to_string(),
            "$ cd e".to_string(),
            "$ ls".to_string(),
            "584 i".to_string(),
            "$ cd ..".to_string(),
            "$ cd ..".to_string(),
            "$ cd d".to_string(),
            "$ ls".to_string(),
            "4060174 j".to_string(),
            "8033020 d.log".to_string(),
            "5626152 d.ext".to_string(),
            "7214296 k".to_string(),
        ];

        assert_eq!(day_07_2(&input), 24933642);
    }
}
