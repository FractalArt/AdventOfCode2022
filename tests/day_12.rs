use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_day_12() {
    let mut data = String::new();
    let _ = File::open("data/day_12.txt")
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();
    let task_1 = aoc2022::day_12::day_12_1(&data);
    assert_eq!(task_1, 456);
}
