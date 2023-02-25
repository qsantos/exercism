pub fn encrypt(input: &str) -> String {
    // normalize
    let letters: Vec<char> = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    if letters.is_empty() {
        return String::new();
    }

    // find dimensions
    let mut c = 1;
    while c * c < letters.len() {
        c += 1;
    }
    let c = c;
    let r = if c * (c - 1) >= letters.len() {
        c - 1
    } else {
        c
    };

    // transpose
    let rows: Vec<&[char]> = letters.chunks(c).collect();
    let mut ret = Vec::new();
    for i in 0..c {
        for &row in &rows {
            ret.push(row.get(i).copied().unwrap_or(' '));
        }
    }

    // cut into chunks
    let chunks: Vec<&[char]> = ret.chunks(r).collect();
    chunks.join(&' ').iter().collect()
}
