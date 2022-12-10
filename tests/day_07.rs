use aoc2022::{self, read_data};

#[test]
fn test_day_07() {
    let data = read_data::<String, _>("data/day_07.txt").unwrap();
    let task_1 = aoc2022::day_07::day_07_1(&data);
    assert_eq!(task_1, 1444896);
    let task_2 = aoc2022::day_07::day_07_2(&data);
    assert_eq!(task_2, 404395);
}
