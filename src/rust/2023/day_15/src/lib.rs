use aoc_common::string_utils::StringManipulation;

fn hash(input: &str) -> u64 {
    input.chars().fold(0, |acc, c| ((acc + (c as u64)) * 17) % 256)
}

struct Buckets<const N: usize, T> {
    buckets: Vec<Vec<T>>,
    hash_func: fn(&str) -> u64,
}

impl Buckets<256, (String, u64)> {
    fn add(&mut self, key: String, value: u64) {
        let (b, p) = self.get_position(&key);
        match p {
            None => { self.buckets[b].push((key, value)) }
            Some(p) => { self.buckets[b][p].1 = value }
        }
    }

    fn get_position(&self, key: &String) -> (usize, Option<usize>) {
        let hash = (self.hash_func)(key) as usize;
        (hash, self.buckets[hash].iter().position(|(k, _)| k == key))
    }

    fn remove(&mut self, key: &String) {
        let (b, p) = self.get_position(&key);
        match p {
            None => {}
            Some(x) => {
                self.buckets[b].remove(x);
            }
        }
    }
}

pub fn solve_1(input: &str) -> String {
    format!("{}", input.split(',').map(hash).sum::<u64>())
}

pub fn solve_2(input: &str) -> String {
    let mut bucket = Buckets { buckets: Vec::<Vec<(String, u64)>>::new(), hash_func: hash };
    bucket.buckets.resize(256, Vec::<(String, u64)>::new());
    for instruction in input.split(',').map(|s| s.chars().filter(|c| *c != '\n').collect::<String>()) {
        let (label, value, op) = if instruction.ends_with('-') {
            (String::from(&instruction[..instruction.len() - 1]), 0, 0)
        } else {
            let [l, v] = instruction.split_in::<2>("=");
            (String::from(l), v.parse().unwrap(), 1)
        };
        if op == 0 {
            bucket.remove(&label);
        } else {
            bucket.add(label, value);
        }
    }
    let mut total = 0;
    for (i, b) in bucket.buckets.iter().enumerate() {
        for (j, (_, v)) in b.iter().enumerate() {
            total += (i as u64 + 1) * (j as u64 + 1) * (*v);
        }
    }

    format!("{}", total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    const TEST_INPUT_2: &str = TEST_INPUT_1;

    #[test]
    fn solves_1() {
        assert_eq!(solve_1(TEST_INPUT_1), "1320")
    }

    #[test]
    fn solves_2() {
        assert_eq!(solve_2(TEST_INPUT_2), "145")
    }
}