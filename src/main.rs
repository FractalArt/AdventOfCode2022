use aoc2022::day_07::day_07;

fn main() {
    let input = vec![
        "$ cd /".to_string(),
        "$ ls".to_string(),
        "dir a".to_string(),
        "14848514 b.txt".to_string(),
        "8504156 c.dat".to_string(),
        "dir d".to_string(),
        "$ cd a".to_string(),
        "$ ls".to_string(),
        "dir e".to_string(),
        "29116 f".to_string(),
        "2557 g".to_string(),
        "62596 h.lst".to_string(),
        "$ cd e".to_string(),
        "$ ls".to_string(),
        "584 i".to_string(),
        "$ cd ..".to_string(),
        "$ cd ..".to_string(),
        "$ cd d".to_string(),
        "$ ls".to_string(),
        "4060174 j".to_string(),
        "8033020 d.log".to_string(),
        "5626152 d.ext".to_string(),
        "7214296 k".to_string(),
    ];

    //assert_eq!(day_07(&input), 95437);
    for i in (1..10).rev() {
        println!("{i}");
    }
}
