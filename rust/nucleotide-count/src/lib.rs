use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let counts = nucleotide_counts(dna)?;
    counts.get(&nucleotide).copied().ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::from([('A', 0usize), ('C', 0usize), ('G', 0usize), ('T', 0usize)]);
    for c in dna.chars() {
        match c {
            'A' | 'C' | 'G' | 'T' => *counts.get_mut(&c).unwrap() += 1,
            _ => return Result::Err(c),
        }
    }
    Result::Ok(counts)
}
