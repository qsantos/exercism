fn rotate_letter(c: char, key: u8) -> char {
    match c {
        'A'..='Z' => (b'A' + (c as u8 - b'A' + key) % 26) as char,
        'a'..='z' => (b'a' + (c as u8 - b'a' + key) % 26) as char,
        _ => c,
    }
}

pub fn rotate(input: &str, key: i8) -> String {
    let key = key.rem_euclid(26) as u8;
    input.chars().map(|c| rotate_letter(c, key)).collect()
}
