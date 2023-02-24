use modinverse::egcd;

#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const M: i32 = 26;

pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let (g, _, _) = egcd(a, M);
    if g != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let bytes: Vec<u8> = plaintext
        .to_ascii_lowercase()
        .bytes()
        .filter(|&c| c.is_ascii_alphanumeric())
        .map(|c| match c {
            b'a'..=b'z' => b'a' + ((c - b'a') as i32 * a + b).rem_euclid(M) as u8,
            b => b,
        })
        .collect();
    let chunks: Vec<&[u8]> = bytes.chunks(5).collect();
    let ciphertext = String::from_utf8(chunks.join(&b' ')).unwrap();
    Ok(ciphertext)
}

pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let (g, inv_a, _) = egcd(a, M);
    if g != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let bytes = ciphertext.bytes().filter(|&c| c != b' ').map(|c| match c {
        b'a'..=b'z' => b'a' + (((c - b'a') as i32 - b) * inv_a).rem_euclid(M) as u8,
        b => b,
    });
    let plaintext = String::from_utf8(bytes.collect()).unwrap();
    Ok(plaintext)
}
