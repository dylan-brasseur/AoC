use aoc_common::math_utils::{Interval, IntervalBehavior, IntervalRange};
use aoc_common::string_utils::StringManipulation;

#[allow(unused_variables)]
pub fn solve_1(input: &str) -> String {
    let mut lines = input.lines().skip_while(|s| s.is_empty());
    let mut seeds: Interval = lines.next().unwrap().right_of(":").extract_numbers().map(|s| IntervalRange::new(s, s + 1)).collect::<Vec<IntervalRange>>().into();
    let mut maps: Vec<Vec<(IntervalRange, i64)>> = Vec::new();
    for l in lines {
        if l.is_empty() {
            continue;
        }
        if l.contains(':') {
            maps.push(Vec::new());
            continue;
        }
        let nums: [i64; 3] = l.extract_numbers().collect::<Vec<i64>>().try_into().unwrap();
        maps.last_mut().unwrap().push((IntervalRange::new(nums[1], nums[1] + nums[2]), nums[0] - nums[1]));
    }
    //println!("Seeds : {:?}", seeds);
    for m in &mut maps {
        m.sort_by(|(a, _), (b, _)| a.start.cmp(&b.start));
        //println!("Map : {:?}", m);
    }

    for m in &maps {
        seeds = seeds.map_to(m);
        //println!("Next : {:?}", &seeds);
    }

    format!("{}", seeds.hull().start)
}

#[allow(unused_variables)]
pub fn solve_2(input: &str) -> String {
    let mut lines = input.lines();
    let seeds_line = lines.next().unwrap().right_of(":").extract_numbers().collect::<Vec<i64>>();
    let mut start = 0;
    let mut seeds: Vec<IntervalRange> = Vec::new();
    for (i, l) in seeds_line.iter().enumerate() {
        if i % 2 == 1 {
            seeds.push(IntervalRange::new(start, start + *l));
            start = 0;
        } else {
            start = *l;
        }
    }
    let mut seeds: Interval = seeds.into();
    let mut maps: Vec<Vec<(IntervalRange, i64)>> = Vec::new();
    for l in lines {
        if l.is_empty() {
            continue;
        }
        if l.contains(':') {
            maps.push(Vec::new());
            continue;
        }
        let nums: [i64; 3] = l.extract_numbers().collect::<Vec<i64>>().try_into().unwrap();
        maps.last_mut().unwrap().push((IntervalRange::new(nums[1], nums[1] + nums[2]), nums[0] - nums[1]));
    }
    //println!("Seeds : {:?}", seeds);
    for m in &mut maps {
        m.sort_by(|(a, _), (b, _)| a.start.cmp(&b.start));
        //println!("Map : {:?}", m);
    }

    for m in &maps {
        seeds = seeds.map_to(m);
        //println!("Next : {:?}", &seeds);
    }

    format!("{}", seeds.hull().start)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    const TEST_INPUT_2: &str = TEST_INPUT_1;

    #[test]
    fn solves_1() {
        assert_eq!(solve_1(TEST_INPUT_1), "35")
    }

    #[test]
    fn solves_2() {
        assert_eq!(solve_2(TEST_INPUT_2), "46")
    }
}