use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let s: HashSet<char> = sentence
        .to_lowercase()
        .chars()
        .filter(|c| ('a'..='z').contains(c))
        .collect();
    s.len() == 26
}
