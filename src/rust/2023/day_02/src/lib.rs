use std::cmp::max;
use std::collections::HashMap;

use regex::Regex;


pub fn solve_1(input: &str) -> String {
    let max_values: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let re = Regex::new("([0-9]*).(blue|red|green)").unwrap();
    let games = input.lines().map(|line| line.split(":").last().unwrap_or(""));
    let mut total = 0;
    for (i, g) in games.enumerate() {
        let draws = g.split(";");
        match draws.flat_map(|draw| re.captures_iter(draw)).find_map(|c| {
            let (_, [n, color]) = c.extract();
            max_values.get(color).unwrap_or(&(0u32)).lt(&n.parse::<u32>().unwrap_or(0u32)).then(|| n)
        }) {
            Some(_) => {}
            _ => {
                total += i + 1;
            }
        }
    }
    format!("{}", total)
}

pub fn solve_2(input: &str) -> String {
    let re = Regex::new("([0-9]*).(blue|red|green)").unwrap();
    let games = input.lines().map(|line| line.split(":").last().unwrap_or(""));
    let indices: HashMap<&str, usize> = HashMap::from([("red", 0), ("green", 1), ("blue", 2)]);
    let mut total = 0;
    for g in games {
        let draws = g.split(";");
        total += draws.flat_map(|draw| re.captures_iter(draw)).map(|c| {
            let (_, [n, color]) = c.extract();
            (n.parse::<u32>().unwrap(), indices.get(color).unwrap())
        }).fold([0, 0, 0], |mut acc, (n, c)| {
            acc[*c] = max(acc[*c], n);
            acc
        }).iter().fold(1, |acc, x| acc * x)
    }
    format!("{}", total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str =
        r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    const TEST_INPUT_2: &str = TEST_INPUT_1;

    #[test]
    fn solves_1() {
        assert_eq!(solve_1(TEST_INPUT_1), "8")
    }

    #[test]
    fn solves_2() {
        assert_eq!(solve_2(TEST_INPUT_2), "2286")
    }
}