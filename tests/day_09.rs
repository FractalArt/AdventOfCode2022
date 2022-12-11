use aoc2022::{self, read_data};

#[test]
fn test_day_09() {
    let data = read_data::<String, _>("data/day_09.txt").unwrap();
    let task_1 = aoc2022::day_09::day_09(&data, 2);
    assert_eq!(task_1, 6030);
    let task_2 = aoc2022::day_09::day_09(&data, 10);
    assert_eq!(task_2, 2545);
}
