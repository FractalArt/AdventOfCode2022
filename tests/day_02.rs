use aoc2022::{self, read_data};

#[test]
fn test_day_02() {
    let data = read_data::<String, _>("data/day_02.txt").unwrap();
    let task_2 = aoc2022::day_02::day_02(&data);
    assert_eq!(task_2, 13924);
}
