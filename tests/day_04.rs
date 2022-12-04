use aoc2022::{self, read_data};

#[test]
fn test_day_04() {
    let data = read_data::<String, _>("data/day_04.txt").unwrap();
    let task_1 = aoc2022::day_04::day_04(&data, false);
    assert_eq!(task_1, 605);
    let task_2 = aoc2022::day_04::day_04(&data, true);
    assert_eq!(task_2, 914);
}
