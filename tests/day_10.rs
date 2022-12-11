use aoc2022::{self, read_data};

#[test]
fn test_day_10() {
    let data = read_data::<String, _>("data/day_10.txt").unwrap();
    let task_1 = aoc2022::day_10::day_10(&data);
    assert_eq!(task_1, 15880);
}
