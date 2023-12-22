use std::ops::Range;
use aoc_common::aoc_utils::not_yet;

fn parse(input:&str, offset_inc: u64) -> Vec<(i64, i64)>{
    let mut offset :u64 = 0;
    let mut galaxies: Vec<(i64, i64)> = Vec::new();
    let mut n_columns = 0;
    for (i, l) in input.lines().enumerate(){
        let mut added= false;
        for (j, c) in l.chars().enumerate(){
            if c == '#'{
                galaxies.push(((i as u64+offset) as i64, j as i64));
                added = true;
            }
        }
        if !added{
            offset+=offset_inc;
        }
        n_columns = l.len();
    }

    for i in (0..n_columns).rev(){
        if galaxies.iter().all(|(_, y)| *y != i as i64){
            for g in &mut galaxies{
                if g.1 > i as i64{
                    *g = (g.0, g.1+1);
                }
            }
        }
    }

    galaxies
}

pub fn solve_1(input: &str) -> String {
    let galaxies = parse(input, 1);
    let mut total=0;
    for i in 0..galaxies.len()-1{
        for j in i+1..galaxies.len(){
            let a = galaxies[i];
            let b = galaxies[j];
            total+= (b.0-a.0).abs() + (b.1-a.1).abs();
        }
    }

    format!("{}", total)
}

pub fn solve_2(input: &str) -> String {
    let galaxies = parse(input, 999999);
    let mut total=0;
    for i in 0..galaxies.len()-1{
        for j in i+1..galaxies.len(){
            let a = galaxies[i];
            let b = galaxies[j];
            total+= (b.0-a.0).abs() + (b.1-a.1).abs();
        }
    }

    format!("{}", total)
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    const TEST_INPUT_2: &str = TEST_INPUT_1;

    #[test]
    fn solves_1() {
        assert_eq!(solve_1(TEST_INPUT_1), "374")
    }

    #[test]
    fn solves_2() {
        assert_eq!(solve_2(TEST_INPUT_2), "")
    }
}