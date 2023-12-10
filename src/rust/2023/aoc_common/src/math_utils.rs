use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use crate::math_utils::IntervalOperationResult::{Multiple, Single};

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

#[derive(Copy, Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct IntervalRange {
    start: i64,
    end: i64,
}

pub struct Interval {
    ranges: Vec<IntervalRange>,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum IntervalOperationResult<T> {
    Single(T),
    Multiple(Vec<T>),
}

pub trait IntervalBehavior {
    fn new(start: i64, end: i64) -> Self;

    fn empty() -> Self;
    fn union(&self, b: &Self) -> IntervalOperationResult<Self> where Self: Sized;
    fn intersect(&self, b: &Self) -> Self;

    fn contains(&self, val: i64) -> bool;

    fn is_empty(&self) -> bool;

    fn collapse(&mut self);
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
        todo!()
    }

    fn contains(&self, val: i64) -> bool {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
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

        if !self.intersect(&b).is_empty() {
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
}

impl Display for IntervalRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end)
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

impl From<[i64; 2]> for IntervalRange {
    fn from(value: [i64; 2]) -> Self {
        IntervalRange::new(value[0], value[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case([0i64, 2i64].into(), [2i64, 4i64].into(), Multiple(vec ! ([0i64, 2i64].into(), [2i64, 4i64].into())))]
    #[test_case([0i64, 2i64].into(), [1i64, 4i64].into(), Single([0i64, 4i64].into()))]
    #[test_case([2i64, 2i64].into(), [1i64, 4i64].into(), Single([1i64, 4i64].into()))]
    #[test_case([0i64, 0i64].into(), [1i64, 4i64].into(), Single([1i64, 4i64].into()))]
    #[test_case([1i64, 4i64].into(), [0i64, 0i64].into(), Single([1i64, 4i64].into()))]
    #[test_case([0i64, 0i64].into(), [3i64, - 1i64].into(), Single([0i64, 0i64].into()))]
    #[test_case([0i64, 0i64].into(), IntervalRange{start: 3i64, end: - 1i64}, Single([0i64, 0i64].into()))]
    fn utils_union_simple_works(a: IntervalRange, b: IntervalRange, expected: IntervalOperationResult<IntervalRange>) {
        println!("{}", a);
        println!("{}", b);
        let u = a.union(&b);
        println!("{:?}", u);
        assert_eq!(u, expected);
    }
}