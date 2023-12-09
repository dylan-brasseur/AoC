use std::collections::{HashMap, HashSet};

use regex::Regex;

use aoc_common::utils::map_to_lines;

struct PuzzleInput {
    symbols: HashMap<(i32, i32), char>,
    parts: HashMap<(i32, i32), (u32, u32)>,
}

fn parse_input(input: &str) -> PuzzleInput {
    let mut puzzle: PuzzleInput = PuzzleInput { symbols: Default::default(), parts: Default::default() };
    let re = Regex::new(r"(?<symbol>[^[0-9].]+)|(?<part>[0-9]+)").unwrap();
    let mut part_id: u32 = 0;
    for (i, l) in map_to_lines(input).enumerate() {
        for c in re.captures_iter(l) {
            match c.name("symbol") {
                None => {}
                Some(x) => {
                    puzzle.symbols.insert((i as i32, x.start() as i32), x.as_str().chars().next().unwrap());
                }
            }
            match c.name("part") {
                None => {}
                Some(x) => {
                    let number = x.as_str().parse::<u32>().unwrap();
                    for j in 0..x.as_str().len() {
                        puzzle.parts.insert((i as i32, (x.start() + j) as i32), (part_id, number));
                    }
                    part_id += 1;
                }
            }
        }
    }
    return puzzle;
}

#[allow(unused_variables)]
pub fn solve_1(input: &str) -> String {
    let puzzle = parse_input(input);
    let mut explored: HashSet<u32> = Default::default();
    let mut total: u32 = 0;
    for ((x, y), _) in puzzle.symbols {
        for (i, j) in [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1), (x - 1, y), (x + 1, y), (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)] {
            match puzzle.parts.get(&(i, j)) {
                None => {}
                Some((id, value)) => {
                    if !explored.contains(id) {
                        total += value;
                        explored.insert(*id);
                    }
                }
            }
        }
    }

    format!("{}", total)
}

#[allow(unused_variables)]
pub fn solve_2(input: &str) -> String {
    let puzzle = parse_input(input);
    let mut explored: HashSet<u32> = Default::default();
    let mut total: u32 = 0;
    for ((x, y), c) in puzzle.symbols {
        if c == '*' {
            let mut found: Vec<u32> = Vec::new();
            for (i, j) in [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1), (x - 1, y), (x + 1, y), (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)] {
                match puzzle.parts.get(&(i, j)) {
                    None => {}
                    Some((id, value)) => {
                        if !explored.contains(id) {
                            explored.insert(*id);
                            found.push(*value);
                        }
                    }
                }
            }
            if found.len() == 2 {
                total += found.get(0).unwrap() * found.get(1).unwrap();
            }
        }
    }

    format!("{}", total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    const TEST_INPUT_2: &str = TEST_INPUT_1;

    #[test]
    fn solves_1() {
        assert_eq!(solve_1(TEST_INPUT_1), "4361")
    }

    #[test]
    fn solves_2() {
        assert_eq!(solve_2(TEST_INPUT_2), "467835")
    }
}