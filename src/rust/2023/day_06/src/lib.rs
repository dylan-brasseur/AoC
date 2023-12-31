use std::iter::zip;
use aoc_common::math_utils::get_roots;

use aoc_common::string_utils::StringManipulation;

#[allow(unused_variables)]
pub fn solve_1(input: &str) -> String {
    let values = input.lines().map(|s| s.right_of(":").extract_numbers().collect()).collect::<Vec<Vec<i64>>>();
    let ms_mm = zip(values.get(0).unwrap(), values.get(1).unwrap());
    let vals = ms_mm.into_iter().map(|(ms, mm)| get_roots(-1.0, *ms as f64, -(*mm as f64)).unwrap()).map(|(r1, r2)| {
        let range = (f64::floor(r2) - f64::ceil(r1)) as u64;
        if f64::floor(r2) != r2 || f64::ceil(r1) != r1 {
            return range + 1;
        }
        return range - 1;
    });
    format!("{}", vals.reduce(|acc, x| acc * x).unwrap_or(0))
}

#[allow(unused_variables)]
pub fn solve_2(input: &str) -> String {
    let values = input.lines().map(|s| s.right_of(":").replace(|c: char| c.is_whitespace(), "").as_str().extract_numbers().next().unwrap()).collect::<Vec<i64>>();
    let (ms, mm) = (*values.get(0).unwrap() as f64, *values.get(1).unwrap() as f64);
    let (r1, r2) = get_roots(-1.0, ms, -mm).unwrap();
    let (r1, r2) = (f64::ceil(r1), f64::floor(r2));
    if (-(r1 * r1) + ms * r1) > mm || (-(r2 * r2) + ms * r2) > mm {
        return format!("{}", (r2 - r1 + 1.0) as u64);
    }
    return format!("{}", (r2 - r1 - 1.0) as u64);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = r"Time:      7  15   30
Distance:  9  40  200";

    const TEST_INPUT_2: &str = TEST_INPUT_1;

    #[test]
    fn solves_1() {
        assert_eq!(solve_1(TEST_INPUT_1), "288")
    }

    #[test]
    fn solves_2() {
        assert_eq!(solve_2(TEST_INPUT_2), "71503")
    }
}