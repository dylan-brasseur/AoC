use aoc_common::aoc_utils::not_yet;

pub fn solve_1(input: &str) -> String {
    not_yet(input)
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