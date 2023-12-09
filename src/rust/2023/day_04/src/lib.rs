use aoc_common::utils::{find_matching_in_sorted, StringManipulation};


#[allow(unused_variables)]
pub fn solve_1(input: &str) -> String {
    let mut total = 0;
    for [w, h] in input.lines().map(|s| s.right_of(":").split_in::<2>("|").map::<fn(&str) -> Vec<i64>, Vec<i64>>(|s| {
        let mut v = s.extract_numbers().collect::<Vec<i64>>();
        v.sort_unstable();
        v
    })){
        match find_matching_in_sorted(&h, &w){
            0 =>{}
            x => {total += 2u32.pow((x-1) as u32);}
        }

    }

    format!("{}", total)
}

#[allow(unused_variables)]
pub fn solve_2(input: &str) -> String {
    let mut total=0;
    let originals = input.lines().map(|s| s.right_of(":").split_in::<2>("|").map::<fn(&str) -> Vec<i64>, Vec<i64>>(|s| {
        let mut v = s.extract_numbers().collect::<Vec<i64>>();
        v.sort_unstable();
        v
    })).collect::<Vec<[Vec<i64>; 2]>>();
    let mut amount = vec![1i64; originals.len()];
    for (i, [w, h]) in originals.iter().enumerate(){
        let cur_amount = *amount.get_mut(i).unwrap();
        let matches = find_matching_in_sorted(&h, &w);
        for extra in 1..(matches+1){
            *amount.get_mut(i+extra).unwrap()+=cur_amount;
        }
        total+=cur_amount;
    }

    format!("{}", total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    const TEST_INPUT_2: &str = TEST_INPUT_1;

    #[test]
    fn solves_1() {
        assert_eq!(solve_1(TEST_INPUT_1), "13")
    }

    #[test]
    fn solves_2() {
        assert_eq!(solve_2(TEST_INPUT_2), "30")
    }
}