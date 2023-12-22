use itertools::Itertools;

use aoc_common::aoc_utils::not_yet;
use aoc_common::string_utils::StringManipulation;

pub fn parse(input: &str) -> Vec<(Vec<u8>, Vec<u64>)> {
    let mut cases: Vec<(Vec<u8>, Vec<u64>)> = Vec::new();

    for l in input.lines() {
        let [springs, series] = l.split_in::<2>(" ");

        cases.push((springs.chars().map(|c| match c {
            '?' => { 0u8 },
            '.' => { 1u8 },
            '#' => { 2u8 },
            _ => { panic!("Unknown character {}", c) }
        }).collect_vec(), series.split(',').map(|s| s.parse::<u64>().unwrap()).collect_vec()))
    }

    cases
}

pub fn backtrack_count(position: usize, springs: &Vec<u8>, series: &Vec<u64>, matched_group: usize, current_count: usize) -> u64 {
    if position >= springs.len() {
        if current_count == 0 {
            if matched_group == series.len() {
                1
            } else {
                0
            }
        } else if matched_group != series.len() - 1 || current_count != (*series.last().unwrap() as usize) {
            0
        } else {
            1
        }
    } else if matched_group == series.len() {
        return if springs[position..].iter().all(|c| *c == 0 || *c == 1) {
            1
        } else {
            0
        }
    } else {
        match springs[position] {
            0 => {
                (if current_count == 0 {
                    backtrack_count(position + 1, springs, series, matched_group, current_count)
                } else if matched_group >= series.len() || series[matched_group] != current_count as u64 {
                    0
                } else {
                    backtrack_count(position + 1, springs, series, matched_group + 1, 0)
                }) + (if matched_group >= series.len() || series[matched_group] > current_count as u64 {
                    backtrack_count(position + 1, springs, series, matched_group, current_count + 1)
                } else {
                    0
                })
            }
            1 => {
                if current_count == 0 {
                    backtrack_count(position + 1, springs, series, matched_group, current_count)
                } else if matched_group >= series.len() || series[matched_group] != current_count as u64 {
                    0
                } else {
                    backtrack_count(position + 1, springs, series, matched_group + 1, 0)
                }
            }
            2 => {
                if matched_group >= series.len() || series[matched_group] > current_count as u64 {
                    backtrack_count(position + 1, springs, series, matched_group, current_count + 1)
                } else {
                    0
                }
            }
            _ => {
                panic!("Never")
            }
        }
    }
}

pub fn solve_1(input: &str) -> String {
    let cases = parse(input);
    let mut total = 0;
    for (springs, series) in cases {
        total += backtrack_count(0, &springs, &series, 0, 0);
    }

    format!("{}", total)
}

pub fn solve_2(input: &str) -> String {
    not_yet(input);
    let mut cases = parse(input);
    cases = cases.iter().map(|(springs, series)| ([springs.clone(), springs.clone(), springs.clone(), springs.clone(), springs.clone()].join(&0u8), series.repeat(5))).collect_vec();
    let mut total = 0;
    for (springs, series) in cases {
        let count = backtrack_count(0, &springs, &series, 0, 0);
        total += count;
        println!("{}", count);
    }

    format!("{}", total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    const TEST_INPUT_2: &str = TEST_INPUT_1;

    #[test]
    fn solves_1() {
        assert_eq!(solve_1(TEST_INPUT_1), "21")
    }

    #[test]
    fn solves_2() {
        assert_eq!(solve_2(TEST_INPUT_2), "525152")
    }
}