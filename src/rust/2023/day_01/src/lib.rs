use regex::{Captures, Regex};

const DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn match_to_digits(line: Option<Captures>) -> u32 {
    match line {
        None => 0,
        Some(x) if x.get(1).unwrap().len() == 1 => x.get(1).unwrap().as_str().parse::<u32>().unwrap_or(0),
        Some(x) => (DIGITS.iter().position(|&val| val == x.get(1).unwrap().as_str()).unwrap() + 1) as u32
    }
}

fn solve(input: &str, first: Regex, last: Regex) -> String {
    format!("{}", input.lines().map(|line| 10 * match_to_digits(first.captures(line)) + match_to_digits(last.captures(line))).sum::<u32>())
}

pub fn solve_1(input: &str) -> String {
    let first = Regex::new(r"([0-9])").unwrap();
    let last = Regex::new(r"^.*([0-9])").unwrap();

    solve(input, first, last)
}

pub fn solve_2(input: &str) -> String {
    let first = Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let last = Regex::new(r"^.*([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    solve(input, first, last)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str =
        r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const TEST_INPUT_2: &str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn solves_example_1() {
        assert_eq!(solve_1(TEST_INPUT_1), "142")
    }

    #[test]
    fn solves_example_2() {
        assert_eq!(solve_2(TEST_INPUT_2), "281")
    }
}