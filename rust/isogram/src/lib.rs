use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut counts = HashMap::new();
    for c in candidate.to_lowercase().chars() {
        if !c.is_whitespace() && c != '-' {
            counts.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
    }
    counts.values().all(|c| *c <= 1)
}
