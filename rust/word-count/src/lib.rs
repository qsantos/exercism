use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for word in words.split(|c: char| !c.is_alphanumeric() && c != '\'') {
        if word.is_empty() {
            continue;
        }
        let word = word.trim_matches('\'').to_string().to_lowercase();
        counts.entry(word).and_modify(|e| *e += 1).or_insert(1);
    }
    counts
}
