use std::{fmt::Display, ops::Rem, str::FromStr};

pub struct Matcher<T> {
    predicate: fn(T) -> bool,
    word: String,
}

impl<T> Matcher<T> {
    pub fn new(predicate: fn(T) -> bool, word: &str) -> Matcher<T> {
        Matcher {
            predicate,
            word: String::from_str(word).unwrap(),
        }
    }
}

pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

struct Iter<T, I> {
    fizzy: Fizzy<T>,
    iter: I,
}

impl<T: Copy + Display, I: Iterator<Item = T>> Iterator for Iter<T, I> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(v) => {
                let mut ret = String::new();
                for matcher in &self.fizzy.matchers {
                    if (matcher.predicate)(v) {
                        ret.push_str(&matcher.word);
                    }
                }
                if ret.is_empty() {
                    ret = v.to_string();
                }
                Some(ret)
            }
            None => None,
        }
    }
}

impl<T: Copy + Display> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy {
            matchers: Vec::new(),
        }
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    pub fn apply<I: Iterator<Item = T>>(self, iter: I) -> impl Iterator<Item = String> {
        Iter { fizzy: self, iter }
    }
}

impl<T: Copy + Display> Default for Fizzy<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T: Copy + Display + Rem<Output = T> + PartialEq + From<u8>>() -> Fizzy<T> {
    Fizzy::new()
        .add_matcher(Matcher::new(|n| n % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|n| n % T::from(5) == T::from(0), "buzz"))
}
