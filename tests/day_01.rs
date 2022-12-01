use aoc2022::{self, read_data};

#[test]
fn test_day_1() {
    let data = read_data::<String, _>("data/day_01.txt").unwrap();
    let task_1 = aoc2022::day_01::day_1(&data);
    assert_eq!(task_1, 70369);
}
