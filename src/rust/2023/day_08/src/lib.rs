use std::collections::HashMap;
use std::ops::ControlFlow;

use itertools::Itertools;

pub fn solve_1(input: &str) -> String {
    let mut lines = input.lines();
    let directions: Vec<usize> = lines.next().unwrap_or("").chars().map(|c| match c {
        'L' => 0,
        'R' => 1,
        _ => panic!()
    }).collect_vec();
    let mut mapping: HashMap<&str, [&str; 2]> = HashMap::new();
    let binding = lines.skip_while(|s| s.is_empty()).map(|s| s.chars().filter(|&c| !c.is_whitespace() && c != '(' && c != ')').collect::<String>()).collect_vec();
    for i in binding.iter().map(|s| s.split('=').collect_vec()) {
        let [key, tuple] = i.try_into().unwrap();
        mapping.insert(key, tuple.split(',').collect_vec().try_into().unwrap());
    }
    let answer = match directions.iter().cycle().try_fold(("AAA", 0), |(k, p), d| match mapping[k][*d] {
        "ZZZ" => ControlFlow::Break(("ZZZ", p + 1)),
        x => ControlFlow::Continue((x, p + 1))
    }) {
        ControlFlow::Continue(_) => { panic!() }
        ControlFlow::Break((_, x)) => { x }
    };
    format!("{}", answer)
}

pub fn solve_2(input: &str) -> String {
    let mut lines = input.lines();
    let directions: Vec<usize> = lines.next().unwrap_or("").chars().map(|c| match c {
        'L' => 0,
        'R' => 1,
        _ => panic!()
    }).collect_vec();
    let mut mapping: HashMap<&str, [&str; 2]> = HashMap::new();
    let binding = lines.skip_while(|s| s.is_empty()).map(|s| s.chars().filter(|&c| !c.is_whitespace() && c != '(' && c != ')').collect::<String>()).collect_vec();
    for i in binding.iter().map(|s| s.split('=').collect_vec()) {
        let [key, tuple] = i.try_into().unwrap();
        mapping.insert(key, tuple.split(',').collect_vec().try_into().unwrap());
    }
    let ghosts = mapping.keys().filter(|k| k.ends_with('Z')).cloned().collect_vec();

    let answer = match directions.iter().cycle().try_fold((ghosts, 0), |(k, p), d| {
        let new_k = k.iter().map(|s| mapping[*s][*d]).collect_vec();
        if new_k.iter().all(|s| s.ends_with('Z')) {
            ControlFlow::Break((new_k, p + 1))
        } else {
            ControlFlow::Continue((new_k, p + 1))
        }
    }) {
        ControlFlow::Continue(_) => { panic!() }
        ControlFlow::Break((_, x)) => { x }
    };
    format!("{}", answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_2: &str = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn solves_1() {
        assert_eq!(solve_1(TEST_INPUT_1), "6")
    }

    #[test]
    fn solves_2() {
        assert_eq!(solve_2(TEST_INPUT_2), "6")
    }
}