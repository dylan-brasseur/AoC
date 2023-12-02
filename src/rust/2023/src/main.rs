use std::env;
use aoc_common::{account_type_from_string, get_input, USAGE};


const DAYS: [[fn(&str) -> String; 2]; 1] = [
[day_01::solve_1, day_01::solve_2]
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

    println!("{}", DAYS.get((day-1) as usize).unwrap().get((task-1) as usize).unwrap()(&input));
}