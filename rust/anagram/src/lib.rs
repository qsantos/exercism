use std::collections::{HashMap, HashSet};

fn collate(word: &str) -> HashMap<char, usize> {
    let mut ret = HashMap::new();
    for c in word.to_lowercase().chars() {
        ret.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }
    ret
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_l = word.to_lowercase();
    let word_a = collate(word);
    possible_anagrams
        .iter()
        .filter(|&&candidate| candidate.to_lowercase() != word_l && collate(candidate) == word_a)
        .copied()
        .collect()
}
