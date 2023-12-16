use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use crate::math_utils::IntervalOperationResult::{Multiple, Single};
use gcd::Gcd;

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

pub fn gcd(a: u64, b:u64) -> u64
{
    a.gcd(b)
}

pub fn lcm(a:u64, b:u64) -> u64
{
    (a*b)/gcd(a,b)
}
#[derive(Copy, Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct IntervalRange {
    pub start: i64,
    end: i64,
}

#[derive(Debug)]
pub struct Interval {
    ranges: Vec<IntervalRange>,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum IntervalOperationResult<T> {
    Single(T),
    Multiple(Vec<T>),
}

impl Display for IntervalOperationResult<IntervalRange> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Single(x) => { write!(f, "{}", x) }
            Multiple(x) => {
                write!(f, "[")?;
                for (i, r) in x.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", {}", r)?;
                    } else {
                        write!(f, "{}", r)?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

impl Display for IntervalOperationResult<Interval> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Single(x) => { write!(f, "{}", x) }
            Multiple(x) => {
                write!(f, "[")?;
                for (i, r) in x.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", {}", r)?;
                    } else {
                        write!(f, "{}", r)?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

pub trait IntervalBehavior {
    fn new(start: i64, end: i64) -> Self;

    fn empty() -> Self;
    fn union(&self, b: &Self) -> IntervalOperationResult<Self> where Self: Sized;
    fn intersect(&self, b: &Self) -> Self;

    fn contains(&self, val: i64) -> bool;

    fn is_empty(&self) -> bool;

    fn collapse(&mut self);

    fn shift(&self, amount: i64) -> Self;

    fn difference(&self, other: &Self) -> IntervalOperationResult<Self> where Self: Sized;
}

impl IntervalBehavior for Interval {
    fn new(start: i64, end: i64) -> Self {
        Interval { ranges: vec!(IntervalRange::new(start, end)) }
    }

    fn empty() -> Self {
        Interval { ranges: Vec::new() }
    }

    fn union(&self, b: &Self) -> IntervalOperationResult<Self> {
        let mut interval = Interval { ranges: self.ranges.iter().chain(b.ranges.iter()).cloned().collect() };
        interval.collapse();
        Single(interval)
    }

    fn intersect(&self, b: &Self) -> Self {
        self.ranges.iter().flat_map(|i| b.ranges.iter().map(|j| i.intersect(j))).collect::<Vec<IntervalRange>>().into()
    }

    fn contains(&self, val: i64) -> bool {
        for i in &self.ranges {
            if i.contains(val) {
                return true;
            }
        }
        false
    }

    fn is_empty(&self) -> bool {
        self.ranges.iter().all(|i| i.is_empty())
    }

    fn collapse(&mut self) {
        self.ranges.sort_by(|a, b| a.start.cmp(&b.start));
        let mut final_vec: Vec<IntervalRange> = Vec::new();
        for i in 0..self.ranges.len() {
            if self.ranges[i].is_empty() {
                continue;
            }
            match final_vec.pop() {
                None => { final_vec.push(self.ranges[i]) }
                Some(x) => {
                    match x.union(&self.ranges[i]) {
                        Single(y) => { final_vec.push(y) }
                        Multiple(y) => {
                            final_vec.push(y[0]);
                            final_vec.push(y[1])
                        }
                    }
                }
            }
        }
        self.ranges = final_vec;
    }

    fn shift(&self, amount: i64) -> Self {
        self.ranges.iter().map(|i| i.shift(amount)).collect::<Vec<IntervalRange>>().into()
    }

    fn difference(&self, other: &Self) -> IntervalOperationResult<Self> {
        let int = self.intersect(other);
        if int.is_empty() {
            return Single(self.ranges.clone().into());
        }
        let mut new_ranges: Vec<IntervalRange> = Vec::new();
        for r in &self.ranges {
            let mut r = r.clone();
            for j in &int.ranges {
                match r.difference(&j) {
                    Single(_) => {}
                    Multiple(x) => {
                        new_ranges.push(x[0]);
                        r = x[1].clone();
                    }
                }
                if r.is_empty() {
                    break;
                }
            }
            if !r.is_empty() {
                new_ranges.push(r);
            }
        }
        Single(new_ranges.into())
    }
}

impl IntervalBehavior for IntervalRange {
    fn new(start: i64, end: i64) -> Self {
        let mut r = IntervalRange { start, end };
        r.collapse();
        r
    }

    fn empty() -> Self {
        IntervalRange::new(0, 0)
    }

    fn union(&self, b: &Self) -> IntervalOperationResult<Self> {
        if self.is_empty() {
            if b.is_empty() {
                return Single(IntervalRange::empty());
            }
            return Single(*b);
        } else if b.is_empty() {
            return Single(*self);
        }

        if self.contiguous(b) || !self.intersect(&b).is_empty() {
            return Single(IntervalRange::new(min(self.start, b.start), max(self.end, b.end)));
        }
        if self.start >= b.start {
            return Multiple(vec![*b, *self]);
        }
        Multiple(vec![*self, *b])
    }

    fn intersect(&self, b: &Self) -> Self {
        IntervalRange::new(max(self.start, b.start), min(self.end, b.end))
    }

    fn contains(&self, val: i64) -> bool {
        val >= self.start && val < self.end
    }

    fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    fn collapse(&mut self) {
        if self.is_empty() {
            self.start = 0;
            self.end = 0;
        }
    }

    fn shift(&self, amount: i64) -> Self {
        IntervalRange::new(self.start + amount, self.end + amount)
    }

    fn difference(&self, other: &Self) -> IntervalOperationResult<Self> {
        let int = self.intersect(other);
        if int.is_empty() {
            return Single(IntervalRange::new(self.start, self.end));
        }
        Multiple(vec!(IntervalRange::new(self.start, int.start), IntervalRange::new(int.end, self.end)))
    }
}

impl Display for IntervalRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}[", self.start, self.end)
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, r) in self.ranges.iter().enumerate() {
            if i > 0 {
                write!(f, ", {}", r)?;
            } else {
                write!(f, "{}", r)?;
            }
        }
        write!(f, "]")
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.ranges.eq(&other.ranges)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl From<[i64; 2]> for IntervalRange {
    fn from(value: [i64; 2]) -> Self {
        IntervalRange::new(value[0], value[1])
    }
}

impl IntervalRange {
    pub fn contiguous(&self, other: &IntervalRange) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

impl Interval {
    pub fn hull(&self) -> IntervalRange {
        if self.ranges.is_empty() {
            return IntervalRange::empty();
        }
        IntervalRange::new(self.ranges.first().unwrap().start, self.ranges.last().unwrap().end)
    }

    pub fn map_to(&self, ranges_shifts: &Vec<(IntervalRange, i64)>) -> Interval {
        let mut new_ranges: Vec<IntervalRange> = Vec::new();
        let mut to_remove: Vec<IntervalRange> = Vec::new();
        for (r, s) in ranges_shifts {
            let mut int = self.intersect(&vec!(*r).into());
            if int.is_empty() {
                continue;
            }
            new_ranges.append(&mut int.shift(*s).ranges.clone());
            to_remove.append(&mut int.ranges);
        }
        match self.difference(&to_remove.into()) {
            Single(x) => {
                match x.union(&new_ranges.into()) {
                    Single(x) => x,
                    _ => { panic!("Shouldn't happen") }
                }
            }
            _ => { panic!("Shouldn't happen") }
        }
    }
}

impl From<Vec<IntervalRange>> for Interval {
    fn from(value: Vec<IntervalRange>) -> Self {
        let mut i = Interval { ranges: value };
        i.collapse();
        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case([0i64, 2i64].into(), [2i64, 4i64].into(), Single([0i64, 4i64].into()))]
    #[test_case([0i64, 2i64].into(), [1i64, 4i64].into(), Single([0i64, 4i64].into()))]
    #[test_case([2i64, 2i64].into(), [1i64, 4i64].into(), Single([1i64, 4i64].into()))]
    #[test_case([0i64, 0i64].into(), [1i64, 4i64].into(), Single([1i64, 4i64].into()))]
    #[test_case([1i64, 4i64].into(), [0i64, 0i64].into(), Single([1i64, 4i64].into()))]
    #[test_case([0i64, 0i64].into(), [3i64, - 1i64].into(), Single([0i64, 0i64].into()))]
    #[test_case([0i64, 0i64].into(), IntervalRange{start: 3i64, end: - 1i64}, Single([0i64, 0i64].into()))]
    fn utils_union_simple_works(a: IntervalRange, b: IntervalRange, expected: IntervalOperationResult<IntervalRange>) {
        println!("A : {}", a);
        println!("B : {}", b);
        let u = a.union(&b);
        println!("U : {}", u);
        assert_eq!(u, expected);
    }

    #[test_case([0i64, 2i64].into(), [2i64, 4i64].into(), IntervalRange::empty())]
    #[test_case([0i64, 2i64].into(), [1i64, 4i64].into(), IntervalRange::new(1, 2))]
    #[test_case([0i64, 2i64].into(), [0i64, 2i64].into(), IntervalRange::new(0, 2))]
    #[test_case([0i64, 4i64].into(), [1i64, 3i64].into(), IntervalRange::new(1, 3))]
    #[test_case([1i64, 3i64].into(), [0i64, 4i64].into(), IntervalRange::new(1, 3))]
    #[test_case([1i64, 2i64].into(), IntervalRange{start: 3i64, end: - 1i64}, IntervalRange::empty())]
    fn utils_intersection_simple_works(a: IntervalRange, b: IntervalRange, expected: IntervalRange) {
        println!("A : {}", a);
        println!("B : {}", b);
        let i = a.intersect(&b);
        println!("I : {}", i);
        assert_eq!(i, expected);
    }

    #[test_case(Interval::new(0, 2), Interval::new(2, 4), Single(Interval::new(0, 4)))]
    #[test_case(Interval::new(0, 2), Interval::new(3, 4), Single(Interval{ranges: vec ! (IntervalRange::new(0, 2), IntervalRange::new(3, 4))}))]
    #[test_case(Interval{ranges: vec ! (IntervalRange::new(0, 2), IntervalRange::new(3, 4))}, Interval{ranges: vec ! (IntervalRange::new(5, 6), IntervalRange::new(7, 8))}, Single(Interval{ranges: vec ! (IntervalRange::new(0, 2), IntervalRange::new(3, 4), IntervalRange::new(5, 6), IntervalRange::new(7, 8))}))]
    #[test_case(Interval{ranges: vec ! (IntervalRange::new(0, 2), IntervalRange::new(3, 4))}, Interval{ranges: vec ! (IntervalRange::new(4, 6), IntervalRange::new(7, 8))}, Single(Interval{ranges: vec ! (IntervalRange::new(0, 2), IntervalRange::new(3, 6), IntervalRange::new(7, 8))}))]
    #[test_case(Interval{ranges: vec ! (IntervalRange::new(0, 8))}, Interval{ranges: vec ! (IntervalRange::new(5, 6), IntervalRange::new(7, 8))}, Single(Interval{ranges: vec ! (IntervalRange::new(0, 8))}))]
    #[test_case(Interval{ranges: vec ! (IntervalRange::new(0, 0))}, Interval{ranges: vec ! (IntervalRange::new(5, 6), IntervalRange::new(7, 8))}, Single(Interval{ranges: vec ! (IntervalRange::new(5, 6), IntervalRange::new(7, 8))}))]
    fn utils_union_works(a: Interval, b: Interval, expected: IntervalOperationResult<Interval>) {
        println!("A : {}", a);
        println!("B : {}", b);
        let u = a.union(&b);
        println!("U : {}", u);
        assert_eq!(u, expected);
    }

    #[test_case(Interval::new(0, 2), Interval::new(2, 4), Interval::empty())]
    #[test_case(Interval{ranges: vec ! (IntervalRange::new(0, 2), IntervalRange::new(3, 4))}, Interval{ranges: vec ! (IntervalRange::new(5, 6), IntervalRange::new(7, 8))}, Interval::empty())]
    #[test_case(Interval{ranges: vec ! (IntervalRange::new(0, 2), IntervalRange::new(3, 4))}, Interval{ranges: vec ! (IntervalRange::new(4, 6), IntervalRange::new(7, 8))}, Interval::empty())]
    #[test_case(Interval{ranges: vec ! (IntervalRange::new(0, 2), IntervalRange::new(3, 5))}, Interval{ranges: vec ! (IntervalRange::new(4, 6), IntervalRange::new(7, 8))}, Interval::new(4, 5))]
    #[test_case(Interval{ranges: vec ! (IntervalRange::new(0, 8))}, Interval{ranges: vec ! (IntervalRange::new(5, 6), IntervalRange::new(7, 8))}, vec ! (IntervalRange::new(5, 6), IntervalRange::new(7, 8)).into())]
    #[test_case(Interval{ranges: vec ! (IntervalRange::new(0, 8))}, Interval{ranges: vec ! (IntervalRange::new(5, 6), IntervalRange::new(7, 16))}, vec ! (IntervalRange::new(5, 6), IntervalRange::new(7, 8)).into())]
    fn utils_intersection_works(a: Interval, b: Interval, expected: Interval) {
        println!("A : {}", a);
        println!("B : {}", b);
        let i = a.intersect(&b);
        println!("I : {}", i);
        assert_eq!(i, expected);
    }

    #[test_case(Interval::new(0, 2), & vec ! ((IntervalRange::new(1, 2), 2)), vec ! (IntervalRange::new(0, 1), IntervalRange::new(3, 4)).into())]
    #[test_case(Interval::new(0, 2), & vec ! ((IntervalRange::new(2, 3), 2)), vec ! (IntervalRange::new(0, 2)).into())]
    #[test_case(Interval::new(0, 10), & vec ! ((IntervalRange::new(2, 3), 10), (IntervalRange::new(5, 7), 10)), vec ! (IntervalRange::new(0, 2), IntervalRange::new(3, 5), IntervalRange::new(7, 10), IntervalRange::new(12, 13), IntervalRange::new(15, 17)).into())]
    #[test_case(Interval::new(0, 10), & vec ! ((IntervalRange::new(2, 3), 10), (IntervalRange::new(5, 7), 10), (IntervalRange::new(9, 10), 10)), vec ! (IntervalRange::new(0, 2), IntervalRange::new(3, 5), IntervalRange::new(7, 9), IntervalRange::new(12, 13), IntervalRange::new(15, 17), IntervalRange::new(19, 20)).into())]
    fn utils_map_to(a: Interval, map: &Vec<(IntervalRange, i64)>, expected: Interval) {
        println!("A : {}", a);
        println!("B : {:?}", map);
        let i = a.map_to(map);
        println!("I : {}", i);
        assert_eq!(i, expected);
    }
}