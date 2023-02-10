use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Palindrome(u64);

impl Palindrome {
    pub fn new(value: u64) -> Option<Palindrome> {
        let rev_digits: String = value.to_string().chars().rev().collect();
        let rev_value: u64 = rev_digits.parse().unwrap();
        if rev_value == value {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    pub fn into_inner(self) -> u64 {
        self.0
    }
}

struct IncreasingProducts {
    queue: BinaryHeap<(Reverse<u64>, u64, u64)>,
    max: u64,
}

impl IncreasingProducts {
    fn new(min: u64, max: u64) -> Self {
        let mut queue = BinaryHeap::new();
        for v in min..=max {
            queue.push((Reverse(v * v), v, v));
        }
        IncreasingProducts { queue, max }
    }
}

impl Iterator for IncreasingProducts {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if let Some((p, a, b)) = self.queue.pop() {
            if b < self.max {
                let b = b + 1;
                self.queue.push((Reverse(a * b), a, b));
            }
            Some(p.0)
        } else {
            None
        }
    }
}

struct DecreasingProducts {
    queue: BinaryHeap<(u64, u64, u64)>,
}

impl DecreasingProducts {
    fn new(min: u64, max: u64) -> Self {
        let mut queue = BinaryHeap::new();
        for v in min..=max {
            queue.push((v * v, v, v));
        }
        DecreasingProducts { queue }
    }
}

impl Iterator for DecreasingProducts {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if let Some((p, a, b)) = self.queue.pop() {
            if b > 0 {
                let b = b - 1;
                self.queue.push((a * b, a, b));
            }
            Some(p)
        } else {
            None
        }
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let smallest = IncreasingProducts::new(min, max)
        .into_iter()
        .flat_map(Palindrome::new)
        .next();
    let largest = DecreasingProducts::new(min, max)
        .into_iter()
        .flat_map(Palindrome::new)
        .next();
    smallest.zip(largest)
}
