use aoc_common::string_utils::StringManipulation;
use itertools::Itertools;

pub fn get_next(vec: Vec<i64>) -> i64{
    if vec.iter().all_equal(){
        *vec.last().unwrap()
    }else {
        let mut next: Vec<i64> = Vec::with_capacity(vec.len()-1);
        for i in 0..vec.len()-1{
            next.push(vec[i+1]-vec[i]);
        }
        return vec.last().unwrap()+get_next(next);
    }
}

pub fn solve_1(input: &str) -> String {
    let lines = input.lines().map(|s| s.extract_numbers().collect_vec());
    let mut total = 0;
    for l in lines{
        total+=get_next(l);
    }

    format!("{}", total)
}

pub fn solve_2(input: &str) -> String {
    let lines = input.lines().map(|s| {let mut v = s.extract_numbers().collect_vec(); v.reverse(); v});
    let mut total = 0;
    for l in lines{
        total+=get_next(l);
    }

    format!("{}", total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    const TEST_INPUT_2: &str = TEST_INPUT_1;

    #[test]
    fn solves_1() {
        assert_eq!(solve_1(TEST_INPUT_1), "114")
    }

    #[test]
    fn solves_2() {
        assert_eq!(solve_2(TEST_INPUT_2), "2")
    }
}