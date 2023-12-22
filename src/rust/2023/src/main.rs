use std::env;

use aoc_common::aoc_utils::{account_type_from_string, get_input, USAGE};

const DAYS: [[fn(&str) -> String; 2]; 25] = [
    [day_01::solve_1, day_01::solve_2],
    [day_02::solve_1, day_02::solve_2],
    [day_03::solve_1, day_03::solve_2],
    [day_04::solve_1, day_04::solve_2],
    [day_05::solve_1, day_05::solve_2],
    [day_06::solve_1, day_06::solve_2],
    [day_07::solve_1, day_07::solve_2],
    [day_08::solve_1, day_08::solve_2],
    [day_09::solve_1, day_09::solve_2],
    [day_10::solve_1, day_10::solve_2],
    [day_11::solve_1, day_11::solve_2],
    [day_12::solve_1, day_12::solve_2],
    [day_13::solve_1, day_13::solve_2],
    [day_14::solve_1, day_14::solve_2],
    [day_15::solve_1, day_15::solve_2],
    [day_16::solve_1, day_16::solve_2],
    [day_17::solve_1, day_17::solve_2],
    [day_18::solve_1, day_18::solve_2],
    [day_19::solve_1, day_19::solve_2],
    [day_20::solve_1, day_20::solve_2],
    [day_21::solve_1, day_21::solve_2],
    [day_22::solve_1, day_22::solve_2],
    [day_23::solve_1, day_23::solve_2],
    [day_24::solve_1, day_24::solve_2],
    [day_25::solve_1, day_25::solve_2],
];

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
    assert!(day as usize <= DAYS.len());
    assert!(task <= 2);

    println!("{}", DAYS.get((day - 1) as usize).unwrap().get((task - 1) as usize).unwrap()(&input));
}

#[cfg(test)]
mod tests {
    use test_case::test_matrix;

    use super::*;

    #[test_matrix(["google"], [2023], ["01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25"], [1, 2])]
    fn day(account_type: &str, year: i16, day: &str, task: i8) {
        let day = day.parse::<i8>().unwrap();
        let input = get_input(&account_type_from_string(account_type), &year, &day).unwrap_or_else(|_| panic!("Couldn't find input for {}/{} ({})", year, day, account_type));
        println!("{}", DAYS.get((day - 1) as usize).unwrap().get((task - 1) as usize).unwrap()(&input));
    }
}