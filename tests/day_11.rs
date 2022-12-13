use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_day_11() {
    let mut data = String::new();
    let _ = File::open("data/day_11.txt")
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();
    let task_1 = aoc2022::day_11::day_11_1(&data, 20);
    assert_eq!(task_1, 102399);
    let task_2 = aoc2022::day_11::day_11_2(&data, 10000);
    assert_eq!(task_2, 23641658401);
}
