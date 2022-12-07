//! # Advent of Code 2022 - Day 7
//!
//! This module contains the solution of the [second day's challenges](https://adventofcode.com/2022/day/7).
use std::collections::HashMap;
use std::collections::VecDeque;

type DirTree = VecDeque<String>;

fn directory_size(
    name: &str,
    file_sizes: &HashMap<String, usize>,
    sub_dirs: &HashMap<String, VecDeque<String>>,
) -> usize {
    *file_sizes.get(name).unwrap_or(&0)
        + if let Some(dir) = sub_dirs.get(name) {
            dir.iter()
                .map(|dir| directory_size(dir, file_sizes, sub_dirs))
                .sum::<usize>()
        } else {
            0
        }
}

/// The solution to task 1 of day 7.
pub fn day_07(data: &[String]) -> usize {
    let mut parent_dirs = DirTree::new();
    let mut sub_dirs = HashMap::<String, VecDeque<String>>::new();
    let mut file_sizes = HashMap::<String, usize>::new();
    let mut current_dir = "/".to_string();

    for line in data {
        let split: Vec<_> = line.split(' ').collect();

        match (split[0], split[1]) {
            ("$", "cd") => match split[2] {
                ".." => {
                    current_dir = parent_dirs.pop_back().unwrap();
                }
                "/" => {
                    parent_dirs = DirTree::new();
                    current_dir = "/".into();
                }
                d => {
                    sub_dirs
                        .entry(current_dir.clone())
                        .or_default()
                        .push_back(d.into());
                    parent_dirs.push_back(current_dir);
                    current_dir = d.into();
                }
            },
            ("$", "ls") => {}
            ("dir", d) => {
                sub_dirs
                    .entry(d.into())
                    .or_default()
                    .push_back(Default::default());
            }
            (size, _) => {
                *file_sizes.entry(current_dir.clone()).or_insert(0) +=
                    size.parse::<usize>().unwrap();
            }
        }
    }

    println!("GOT HERE");
    println!("sub_dirs: {:?}", &sub_dirs);
    println!("file_sizes: {:?}", &file_sizes);

    sub_dirs
        .keys()
        .map(|x| dbg!(x))
        .map(|key| directory_size(key, &file_sizes, &sub_dirs))
        .map(|size| dbg!(size))
        .filter(|&size| size <= 100000)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_06_dir_size() {
        let sub_dirs: HashMap<String, VecDeque<String>> = vec![
            ("a".into(), VecDeque::from(["e".into()])),
            ("/".into(), VecDeque::from(["d".into(), "a".into()])),
            ("e".into(), VecDeque::from([])),
            ("d".into(), VecDeque::from([])),
        ]
        .into_iter()
        .collect();

        let file_sizes: HashMap<String, usize> = vec![
            ("/".into(), 23352670),
            ("a".into(), 94269),
            ("e".into(), 584),
            ("d".into(), 24933642),
        ]
        .into_iter()
        .collect();

        assert_eq!(directory_size("d", &file_sizes, &sub_dirs), 24933642);
        assert_eq!(directory_size("e", &file_sizes, &sub_dirs), 584);
        assert_eq!(directory_size("a", &file_sizes, &sub_dirs), 94853);
        assert_eq!(directory_size("/", &file_sizes, &sub_dirs), 48381165);
    }

    #[test]
    fn test_day_06_1() {
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

        assert_eq!(day_07(&input), 95437);
    }
}
