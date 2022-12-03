use aoc2022::{self, read_data};

#[test]
fn test_day_03() {
    let data = read_data::<String, _>("data/day_03.txt").unwrap();
    let task_1 = aoc2022::day_03::day_03_1(&data);
    assert_eq!(task_1, 7568);
    let task_2 = aoc2022::day_03::day_03_2(&data);
    assert_eq!(task_2, 2780);
}
