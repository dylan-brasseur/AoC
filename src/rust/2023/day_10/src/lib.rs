use std::collections::HashMap;
use aoc_common::aoc_utils::not_yet;

const NORTH:u8 = 0b1000;
const SOUTH:u8 = 0b0100;
const WEST:u8 = 0b0010;
const EAST:u8 = 0b0001;

const NOTHING:u8 = 0;

const ALL:u8 = NORTH+SOUTH+WEST+EAST;

fn get_mapping() -> HashMap<char, u8>{
    let mut map: HashMap<char, u8> = HashMap::new();
    map.insert('|', NORTH+SOUTH);
    map.insert('-', EAST+WEST);
    map.insert('L', NORTH+EAST);
    map.insert('J', NORTH+WEST);
    map.insert('7', SOUTH+WEST);
    map.insert('F', SOUTH+EAST);
    map.insert('S', ALL);
    map.insert('.', NOTHING);

    map
}

fn get_next(current: (usize, usize), previous: (usize, usize), grid: &Vec<Vec<u8>>) -> (usize, usize){
    let curr_value = grid[current.0][current.1];
    if curr_value & NORTH > 0{
        match current.0.checked_sub(1) {
            None => {}
            Some(x) => {
                if previous != (x, current.1) && grid[x][current.1] & SOUTH > 0{
                    return (x, current.1)
                }
            }
        }
    }
    if curr_value & SOUTH > 0{
        match current.0 + 1 {
            x if x >= grid.len() => {}
            x => {
                if previous != (x, current.1) && grid[x][current.1] & NORTH > 0{
                    return (x, current.1)
                }
            }
        }
    }
    if curr_value & WEST > 0{
        match current.1.checked_sub(1) {
            None => {}
            Some(x) => {
                if previous != (current.0, x) && grid[current.0][x] & EAST > 0{
                    return (current.0, x)
                }
            }
        }
    }
    if curr_value & EAST > 0{
        match current.1+1 {
            x if x >= grid[current.0].len() => {}
            x => {
                if previous != (current.0, x) && grid[current.0][x] & WEST > 0{
                    return (current.0, x)
                }
            }
        }
    }

    panic!()
}

pub fn solve_1(input: &str) -> String {
    let mapping = get_mapping();
    let grid = input.lines().map(|s| s.chars().map(|c| mapping[&c]).collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>();
    let mut x:usize = 0;
    let mut y:usize = 0;
    for (i, l) in grid.iter().enumerate(){
        match l.iter().position(|u| *u == ALL){
            None => {}
            Some(j) => {
                x = i;
                y = j;
                break;
            }
        }
    };
    let mut previous = (x, y);
    let mut current = (x, y);
    let mut distance = 1u64;
    println!("{:?}", &current);
    current = get_next(current, previous, &grid);
    println!("{:?}", &current);
    while current != (x, y){
        let tmp = current;
        distance +=1;
        current = get_next(current, previous, &grid);
        previous = tmp;
        println!("{:?}", &current);
    }

    format!("{}", distance/2)
}

pub fn solve_2(input: &str) -> String {
    not_yet(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = r"-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    const TEST_INPUT_1_1: &str = r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    const TEST_INPUT_2: &str = TEST_INPUT_1;

    #[test]
    fn solves_1() {
        assert_eq!(solve_1(TEST_INPUT_1), "4")
    }

    #[test]
    fn solves_1_1() {
        assert_eq!(solve_1(TEST_INPUT_1_1), "8")
    }

    #[test]
    fn solves_2() {
        assert_eq!(solve_2(TEST_INPUT_2), "")
    }
}