use aoc2022::{self, read_data};

#[test]
fn test_day_01() {
    let data = read_data::<String, _>("data/day_01.txt").unwrap();
    let task_1 = aoc2022::day_01::day_1(&data, 1);
    assert_eq!(task_1, 70369);
    let task_2 = aoc2022::day_01::day_1(&data, 3);
    assert_eq!(task_2, 203002);
}
