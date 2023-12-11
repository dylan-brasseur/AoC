use std::cmp::Ordering;
use std::iter::zip;
use std::ops::ControlFlow;

use itertools::Itertools;

use aoc_common::string_utils::StringManipulation;

const CARDS: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
const CARDS_J: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

#[derive(Debug)]
struct HandBid {
    hand: [u8; 5],
    bid: i64,
    kind: u8,
}


impl HandBid {
    pub fn new(l: &str, cards: [char; 13], jokers: bool) -> HandBid {
        let [hand, bid] = l.split_in::<2>(" ");
        let bid = bid.extract_numbers().next().unwrap();
        let hand: [u8; 5] = hand.chars().map(|c| cards.iter().position(|p| *p == c).unwrap() as u8).collect::<Vec<u8>>().try_into().unwrap();
        let mut kind = hand.iter().into_group_map_by(|&x| x)
            .into_iter()
            .map(|(k, v)| (*k, v.len() as u32))
            .collect::<Vec<(u8, u32)>>();
        kind.sort_unstable_by(|(a, v), (b, v2)| {
            if jokers {
                if *a == 0u8 {
                    return Ordering::Less;
                }
                if *b == 0u8 {
                    return Ordering::Greater;
                }
            }
            v2.cmp(v)
        });
        if jokers {
            let y = match kind.get(0) {
                Some((x, y1)) => {
                    if *x == 0u8 {
                        *y1
                    } else {
                        0u32
                    }
                },
                None => 0u32
            };
            if y > 0 {
                match kind.get_mut(1) {
                    None => {}
                    Some((_, v)) => {
                        *v = *v + y;
                        kind.remove(0);
                    }
                }
            }
        }
        let kind = match kind[..] {
            [(_, 5)] => { 6 },
            [(_, 4), (_, 1)] => { 5 },
            [(_, 3), (_, 2)] => { 4 },
            [(_, 3), (_, 1), (_, 1)] => { 3 },
            [(_, 2), (_, 2), (_, 1)] => { 2 },
            [(_, 2), (_, 1), (_, 1), (_, 1)] => { 1 },
            _ => { 0 }
        };
        HandBid { hand, bid, kind }
    }
}

fn solve(input: &str, cards: [char; 13], jokers: bool) -> String {
    let mut hands = input.lines().map(|s| HandBid::new(s, cards, jokers)).collect_vec();
    hands.sort_unstable_by(|h, h2| {
        match h.kind.cmp(&h2.kind) {
            Ordering::Less => { Ordering::Less }
            Ordering::Greater => { Ordering::Greater }
            Ordering::Equal => {
                match zip(h.hand, h2.hand).try_for_each(|(h0, h1)| match h0.cmp(&h1) {
                    Ordering::Equal => { ControlFlow::Continue(()) }
                    x => { ControlFlow::Break(x) }
                }) {
                    ControlFlow::Continue(_) => { panic!("Shouldn't happen") }
                    ControlFlow::Break(x) => { x }
                }
            }
        }
    });
    format!("{}", hands.iter().enumerate().fold(0, |acc, (i, h)| acc + (h.bid * (i + 1) as i64)))
}

pub fn solve_1(input: &str) -> String {
    solve(input, CARDS, false)
}

pub fn solve_2(input: &str) -> String {
    solve(input, CARDS_J, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    const TEST_INPUT_2: &str = TEST_INPUT_1;

    #[test]
    fn solves_1() {
        assert_eq!(solve_1(TEST_INPUT_1), "6440")
    }

    #[test]
    fn solves_2() {
        assert_eq!(solve_2(TEST_INPUT_2), "5905")
    }
}