use std::env;
use nom::Slice;
use aoc_common::{account_type_from_string, get_input, USAGE};
use regex::{Regex};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!("{}", USAGE)
    }
    let account = account_type_from_string(&args[1]);
    let year: i16 = (&args[2]).parse().expect(&*("Expected year : ".to_owned() + USAGE));
    let day: i8 = (&args[3]).parse().expect(&*("Expected day : ".to_owned() + USAGE));
    let task: i8 = (&args[4]).parse().unwrap_or(1);

    let input = get_input(&account, &year, &day).expect(&*format!("Couldn't find input for {}/{} ({})", year, day, account));

    println!("{}", match task {
        2 => solve_2(&*input),
        _ => { solve_1(&*input) }
    });
}

fn solve_1(input: &str) -> String {
    let mut sum=0;
    for line in input.split("\n"){
        let mut first = 10;
        let mut last = 10;
        for c in line.chars() {
            if c.is_digit(10){
                if first == 10{
                    first = c.to_digit(10).unwrap_or(0);
                    last = first
                }else {
                    last = c.to_digit(10).unwrap_or(0);
                }
            }
        }
        if first != 10{
            sum += 10*first+last;
        }

    }
    format!("{}", sum)
}

fn solve_2(input: &str) -> String{
    const DIGITS: [&str; 9] = ["one","two", "three", "four", "five", "six" , "seven", "eight", "nine"];
    let re = Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let mut sum=0;
    for mut line in input.split("\n"){
        let mut first = 10;
        let mut last = 10;
        while line.len() > 0{
            let m = re.find(line);
            match m {
                None => { line = ""}
                Some(x) if x.len() == 1 => {
                    if first == 10{
                        first = x.as_str().chars().next().unwrap().to_digit(10).unwrap_or(0);
                        last = first
                    }else {
                        last = x.as_str().chars().next().unwrap().to_digit(10).unwrap_or(0);
                    }

                    line = line.slice(x.start()+1..line.len())
                }
                Some(x) => {
                    let value = DIGITS.iter().position(|&val| val == x.as_str()).unwrap() + 1;
                    if first == 10{
                        first = value as u32;
                        last = first
                    }else {
                        last = value as u32;
                    }

                    line = line.slice(x.start()+1..line.len())
                }
            }
        }
        if first != 10{
            sum += 10*first+last;
        }
    }
    format!("{}", sum)
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
    fn solves_1() {
        assert_eq!(solve_1(TEST_INPUT_1), "142")
    }

    #[test]
    fn solves_2() {
        assert_eq!(solve_2(TEST_INPUT_2), "281")
    }
}