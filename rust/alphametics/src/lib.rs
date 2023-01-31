use std::collections::{HashMap, HashSet};

struct Problem<'a> {
    left_words: Vec<&'a str>,
    right_word: &'a str,
    letters: Vec<char>,
    leading_letters: Vec<char>,
}

impl<'a> Problem<'a> {
    fn from(input: &'a str) -> Option<Self> {
        let (left, right_word) = input.split_once(" == ")?;
        let left_words: Vec<&str> = left.split(" + ").collect();
        let mut letters: HashSet<char> = left_words.iter().flat_map(|word| word.chars()).collect();
        letters.extend(right_word.chars());
        let letters = Vec::from_iter(letters);
        let mut leading_letters: Vec<char> = left_words
            .iter()
            .map(|word| word.chars().next().unwrap())
            .collect();
        leading_letters.push(right_word.chars().next().unwrap());
        Some(Problem {
            left_words,
            right_word,
            letters,
            leading_letters,
        })
    }

    fn eval(&self, word: &str, digit_of_letter: &[u8]) -> u64 {
        let mut ret = 0;
        for c in word.chars() {
            let index = self.letters.iter().position(|&d| d == c).unwrap();
            let digit = digit_of_letter[index];
            ret *= 10;
            ret += digit as u64;
        }
        ret
    }

    fn is_solution(&self, digit_of_letter: &[u8]) -> bool {
        let left_number: u64 = self
            .left_words
            .iter()
            .map(|word| self.eval(word, digit_of_letter))
            .sum();
        let right_number = self.eval(self.right_word, digit_of_letter);
        left_number == right_number
    }

    fn solve_at(
        &mut self,
        digit_of_letter: &mut Vec<u8>,
        used_digits: u32,
    ) -> Option<HashMap<char, u8>> {
        let index = digit_of_letter.len();
        if let Some(letter) = self.letters.get(index) {
            let start = self.leading_letters.contains(&letter).into();
            for digit in start..=9 {
                let pattern = 1u32 << digit;
                if used_digits & pattern != 0 {
                    continue;
                }
                digit_of_letter.push(digit);
                let ret = self.solve_at(digit_of_letter, used_digits | pattern);
                digit_of_letter.pop();
                if ret.is_some() {
                    return ret;
                }
            }
        } else if self.is_solution(digit_of_letter) {
            return Some(HashMap::from_iter(
                self.letters
                    .iter()
                    .copied()
                    .zip(digit_of_letter.iter().copied()),
            ));
        }
        None
    }

    fn solve(&mut self) -> Option<HashMap<char, u8>> {
        let mut digit_of_letter = vec![];
        self.solve_at(&mut digit_of_letter, 0)
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut problem = Problem::from(input)?;
    problem.solve()
}
