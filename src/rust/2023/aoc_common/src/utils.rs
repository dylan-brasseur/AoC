use std::iter::FilterMap;
use std::str::Split;

pub fn map_to_lines<'a>(input: &'a str) -> FilterMap<Split<'a, &'static str>, fn(&'a str) -> Option<&'a str>> {
    input.split("\n").filter_map(|s| s.len().gt(&(0usize)).then(|| s))
}

pub fn split_by_multispace(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
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