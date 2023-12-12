use std::{env, fs};
use std::fmt::Display;
use std::path::Path;

use crate::aoc_utils::AccountType::{GitHub, Google, Reddit, Twitter};

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

pub fn not_yet(input: &str) -> String {
    let _ = input;
    todo!();
}
