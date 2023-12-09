use std::{env, fs};
use std::fmt::Display;
use std::iter::FilterMap;
use std::path::Path;
use std::str::Split;

use regex::Regex;

use crate::AccountType::{GitHub, Google, Reddit, Twitter};

pub fn get_aoc_root() -> String {
    match env::var("AOC_ROOT") {
        Ok(val) => val,
        Err(_) => {
            match env::consts::OS {
                "windows" => r"D:\Dev\AoC\".to_string(),
                "linux" => "/mnt/d/Dev/AoC/".to_string(),
                &_ => "/mnt/d/Dev/AoC/".to_string()
            }
        }
    }
}

pub enum AccountType {
    Google,
    GitHub,
    Reddit,
    Twitter,
}

pub fn account_type_from_string(value: &str) -> AccountType {
    match value.to_ascii_lowercase().as_str() {
        "google" => Google,
        "github" => GitHub,
        "twitter" => Twitter,
        "reddit" => Reddit,
        _ => Google
    }
}

pub const USAGE: &str = "cargo run account_type year day task";

impl Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Google => "google",
            GitHub => "github",
            Reddit => "reddit",
            Twitter => "twitter"
        };
        write!(f, "{}", str)
    }
}

pub fn get_input(account_type: &AccountType, year: &i16, day: &i8) -> std::io::Result<String> {
    let path = Path::new(&get_aoc_root()).join("personal").join("inputs").join(account_type.to_string()).join(year.to_string()).join(format!("{:02}", day)).join("input.txt");
    fs::read_to_string(path)
}

pub fn map_to_lines<'a>(input: &'a str) -> FilterMap<Split<'a, &'static str>, fn(&'a str) -> Option<&'a str>> {
    input.split("\n").filter_map(|s| s.len().gt(&(0usize)).then(|| s))
}

pub fn split_by_multispace(input: &str) -> Vec<&str> {
    let re = Regex::new(r"\s*(\S+)").unwrap();
    re.captures_iter(input).map(|c| {
        let (_, [val]) = c.extract();
        val
    }).collect::<Vec<&str>>()
}

pub fn get_roots(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    if a == 0.0 {
        if b == 0.0 {
            return None;
        }
        return Some((-c / b, -c / b));
    }
    let d = b * b - 4.0 * a * c;
    match d {
        d if d < 0.0 => None,
        d if d > 0.0 => {
            let sqrt_d = f64::sqrt(d);
            let r1 = (-b - sqrt_d) / (2.0 * a);
            let r2 = (-b + sqrt_d) / (2.0 * a);
            if r1 > r2 {
                return Some((r2, r1));
            }
            return Some((r1, r2));
        }
        _ => Some((-b / (2.0 * a), -b / (2.0 * a)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = get_aoc_root();
        if cfg!(windows) {
            assert_eq!(result, r"D:\Dev\AoC\");
        } else {
            assert_eq!(result, r"/mnt/d/Dev/AoC/");
        }
    }
}
