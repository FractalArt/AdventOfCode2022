//! # Advent of Code 2022 - Day 11
//!
//! This module contains the solution of the [eleventh day's challenges](https://adventofcode.com/2022/day/11).
use itertools::Itertools;
use std::collections::LinkedList;

#[derive(Debug, PartialEq)]
enum Operation {
    Plus(usize),
    Times(usize),
    Square,
}

struct Monkey {
    handled_items: usize,
    test: usize,
    true_target: usize,
    false_target: usize,
    items: LinkedList<usize>,
    operation: Operation,
}

impl Monkey {
    fn from_string(s: &str) -> Self {
        let split: Vec<_> = s.split('\n').collect();
        let items: LinkedList<usize> = split[1]
            .split(':')
            .last()
            .unwrap()
            .split(',')
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect();

        let operation_value: Vec<_> = split[2].rsplitn(3, ' ').take(2).collect();
        let operation = match (operation_value[0], operation_value[1]) {
            ("old", _) => Operation::Square,
            (v, "*") => Operation::Times(v.parse::<usize>().unwrap()),
            (v, _) => Operation::Plus(v.parse::<usize>().unwrap()),
        };

        let test = split[3]
            .rsplit_once(' ')
            .unwrap()
            .1
            .trim()
            .parse::<usize>()
            .unwrap();

        let targets: Vec<_> = split[4..=5]
            .iter()
            .map(|s| s.rsplit_once(' ').unwrap().1.parse::<usize>().unwrap())
            .collect();

        Self {
            handled_items: 0,
            test,
            true_target: targets[0],
            false_target: targets[1],
            items,
            operation,
        }
    }

    /// Return the `worry level` of the inspected item
    /// together with the index of the monkey it is thrown to.
    /// To keep the numbers from growing, either a division by 3 is used
    /// or all operations are performed in a finite-field.
    fn inspect_next(&mut self, finite_field: Option<usize>) -> Option<(usize, usize)> {
        if self.items.is_empty() {
            return None;
        }
        self.handled_items += 1;
        let item = self.items.pop_front().unwrap();
        let new_worry_level = if let Some(cardinality) = finite_field {
            match self.operation {
                Operation::Plus(x) => ((item % cardinality) + x) % cardinality,
                Operation::Times(x) => ((item % cardinality) * x) % cardinality,
                Operation::Square => ((item % cardinality) * item) % cardinality,
            }
        } else {
            match self.operation {
                Operation::Plus(x) => (item + x) / 3,
                Operation::Times(x) => (item * x) / 3,
                Operation::Square => item.pow(2) / 3,
            }
        };
        if new_worry_level % self.test == 0 {
            Some((new_worry_level, self.true_target))
        } else {
            Some((new_worry_level, self.false_target))
        }
    }
}

/// The solution of part 1 from day 11.
pub fn day_11_1(data: &str, rounds: usize) -> usize {
    let mut monkeys: Vec<_> = data.split("\n\n").map(Monkey::from_string).collect();
    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            while let Some((item, target_monkey)) = monkeys[m].inspect_next(None) {
                monkeys[target_monkey].items.push_back(item);
            }
        }
    }
    monkeys
        .iter()
        .map(|m| m.handled_items)
        .sorted()
        .rev()
        .take(2)
        .product()
}

/// The solution of part 2 from day 11.
pub fn day_11_2(data: &str, rounds: usize) -> usize {
    let mut monkeys: Vec<_> = data.split("\n\n").map(Monkey::from_string).collect();
    let cardinality = monkeys.iter().map(|m| m.test).product();
    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            while let Some((item, target_monkey)) = monkeys[m].inspect_next(Some(cardinality)) {
                monkeys[target_monkey].items.push_back(item);
            }
        }
    }
    monkeys
        .iter()
        .map(|m| m.handled_items)
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_11_monkey() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0";

        let inputs: Vec<_> = input.split("\n\n").collect();

        let mut monkey = Monkey::from_string(inputs[0]);

        assert_eq!(monkey.items, [79, 98].into());
        assert_eq!(monkey.operation, Operation::Times(19));
        assert_eq!(monkey.test, 23);
        assert_eq!(monkey.true_target, 2);
        assert_eq!(monkey.false_target, 3);

        assert_eq!(monkey.inspect_next(None), Some((500, 3)));
        assert_eq!(monkey.handled_items, 1);
        assert_eq!(monkey.inspect_next(None), Some((620, 3)));
        assert_eq!(monkey.handled_items, 2);
    }

    #[test]
    fn test_day_11_1() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        let mut monkeys: Vec<_> = input.split("\n\n").map(Monkey::from_string).collect();
        // simulate one round

        for m in 0..monkeys.len() {
            while let Some((item, target_monkey)) = monkeys[m].inspect_next(None) {
                monkeys[target_monkey].items.push_back(item);
            }
        }

        assert_eq!(monkeys[0].items, [20, 23, 27, 26].into());
        assert_eq!(monkeys[1].items, [2080, 25, 167, 207, 401, 1046].into());
        assert_eq!(monkeys[2].items, [].into());
        assert_eq!(monkeys[3].items, [].into());
        assert_eq!(monkeys[0].handled_items, 2);
        assert_eq!(monkeys[1].handled_items, 4);
        assert_eq!(monkeys[2].handled_items, 3);
        assert_eq!(monkeys[3].handled_items, 5);

        assert_eq!(day_11_1(input, 20), 10605);
    }

    #[test]
    fn test_day_11_2() {
        let input = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

    Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

    Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

    Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        assert_eq!(day_11_2(input, 10000), 2713310158);
    }
}
