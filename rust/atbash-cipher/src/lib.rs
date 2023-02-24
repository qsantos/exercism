fn code_byte(c: u8) -> u8 {
    match c {
        b'a'..=b'z' => b'a' + 25 - (c - b'a'),
        b => b,
    }
}

pub fn encode(plain: &str) -> String {
    let bytes: Vec<u8> = plain
        .to_ascii_lowercase()
        .bytes()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(code_byte)
        .collect();
    let chunks: Vec<&[u8]> = bytes.chunks(5).collect();
    String::from_utf8(chunks.join(&b' ')).unwrap()
}

pub fn decode(cipher: &str) -> String {
    let bytes = cipher.bytes().filter(|&c| c != b' ').map(code_byte);
    String::from_utf8(bytes.collect()).unwrap()
}
