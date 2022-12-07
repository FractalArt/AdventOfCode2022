use aoc2022::{self, read_data};

#[test]
fn test_day_05() {
    let data = read_data::<String, _>("data/day_05.txt").unwrap();
    let task_1 = aoc2022::day_05::day_05_1(&data);
    assert_eq!(task_1, "QPJPLMNNR");
}
