use aoc2022::{self};
use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_day_06() {
    let mut data = String::new();
    let _ = File::open("./data/day_06.txt")
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();
    let task_1 = aoc2022::day_06::day_06(&data);
    assert_eq!(task_1, 1658);
}
