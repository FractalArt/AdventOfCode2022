//! # Advent of Code 2022 - Day 12
//!
//! This module contains the solution of the [twelfth day's challenges](https://adventofcode.com/2022/day/12).
//!
//! To solve the problem I had to learn about Dijkstra's algorithm which I had never heard of
//! before. In the reference section below, I collect the resources I used to learn.
//!
//! ## References
//! - [Dijkstra's algorithm on Wikipedia](https://en.wikipedia.org/wiki/Dijkstra's_algorithm)
//! - [Dijkstra's algorithm on Computerphile](https://www.youtube.com/watch?v=GazC3A4OQTE)
use std::collections::{HashMap, HashSet};

type Pos = (usize, usize);

#[derive(Clone)]
struct HeightMap {
    start: (usize, usize),
    destination: (usize, usize),
    map: Vec<Vec<u8>>,
}

impl HeightMap {
    fn from_string(s: &str) -> Self {
        let (start, destination, map) = s.lines().enumerate().fold(
            ((0, 0), (0, 0), vec![]),
            |(mut start, mut dest, mut map), (y, line)| {
                let row = line
                    .chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = (x, y);
                            b'a'
                        }
                        'E' => {
                            dest = (x, y);
                            b'z'
                        }
                        c => c as u8,
                    })
                    .collect();
                map.push(row);
                (start, dest, map)
            },
        );
        Self {
            start,
            destination,
            map,
        }
    }

    fn get_neighbours(&self, &(x, y): &Pos) -> Vec<Pos> {
        let mut ret = vec![];
        let val = self.map[y][x];
        if x > 0 && self.map[y][x - 1] < val + 2 {
            ret.push((x - 1, y));
        }
        if y > 0 && self.map[y - 1][x] < val + 2 {
            ret.push((x, y - 1))
        }
        if y < self.map.len() - 1 && self.map[y + 1][x] < val + 2 {
            ret.push((x, y + 1))
        }
        if x < self.map[0].len() - 1 && self.map[y][x + 1] < val + 2 {
            ret.push((x + 1, y))
        }

        ret
    }

    fn get_minimum_steps(&self, start: Pos) -> Option<usize> {
        let mut visited: HashSet<Pos> = HashSet::new();
        let mut fringe: HashMap<Pos, usize> = HashMap::new();
        let (mut current_pos, mut current_weight) = (start, 0);

        while current_pos != self.destination {
            self.get_neighbours(&current_pos)
                .into_iter()
                .filter(|pos| !visited.contains(pos))
                .for_each(|pos| {
                    fringe
                        .entry(pos)
                        .and_modify(|current| {
                            *current = std::cmp::min(*current, current_weight + 1)
                        })
                        .or_insert(current_weight + 1);
                });

            visited.insert(current_pos);

            if let Some(current) = fringe.iter().min_by(|x, y| Ord::cmp(&x.1, &y.1)) {
                current_pos = *current.0;
                current_weight = *current.1;
                fringe.remove(&current_pos);
            } else {
                return None;
            }
        }

        Some(current_weight)
    }
}

/// The solution of part 1 from day 12.
pub fn day_12_1(data: &str) -> usize {
    let height_map = HeightMap::from_string(data);
    height_map.get_minimum_steps(height_map.start).unwrap()
}

/// The solution of part 2 from day 12.
pub fn day_12_2(data: &str) -> usize {
    let height_map = HeightMap::from_string(data);
    height_map
        .map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (x, y, c)))
        .filter(|(_, _, &c)| c == b'a')
        .filter_map(|(x, y, _)| height_map.get_minimum_steps((x, y)))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_day_12_height_map() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let map = HeightMap::from_string(input);

        assert_eq!(map.start, (0, 0));
        assert_eq!(map.destination, (5, 2));
        assert_eq!(map.map[0][0], b'a');
        assert_eq!(map.map[0][1], b'a');
        assert_eq!(map.map[0][2], b'b');
        assert_eq!(map.map[0][3], b'q');
        assert_eq!(map.map[4][7], b'i');

        assert_eq!(
            map.get_neighbours(&(0, 0))
                .into_iter()
                .collect::<HashSet<_>>(),
            HashSet::from([(0, 1), (1, 0)])
        );

        assert_eq!(
            map.get_neighbours(&(1, 1))
                .into_iter()
                .collect::<HashSet<_>>(),
            HashSet::from([(0, 1), (2, 1), (1, 0), (1, 2)])
        );

        assert_eq!(
            map.get_neighbours(&(7, 3))
                .into_iter()
                .collect::<HashSet<_>>(),
            HashSet::from([(7, 2), (7, 4)])
        );
    }

    #[test]
    fn test_day_12_1() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(day_12_1(input), 31);
    }

    #[test]
    fn test_day_12_2() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(day_12_2(input), 29);
    }
}
