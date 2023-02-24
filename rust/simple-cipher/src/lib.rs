use rand::{thread_rng, Rng};

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_lowercase()) {
        return None;
    }
    let bytes = s
        .bytes()
        .zip(key.bytes().cycle())
        .map(|(c, k)| b'a' + ((c - b'a') + (k - b'a')).rem_euclid(26));
    Some(String::from_utf8(bytes.collect()).unwrap())
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_lowercase()) {
        return None;
    }
    let bytes = s
        .bytes()
        .zip(key.bytes().cycle())
        .map(|(c, k)| b'a' + ((c - b'a') + 26 - (k - b'a')).rem_euclid(26));
    Some(String::from_utf8(bytes.collect()).unwrap())
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = thread_rng();
    let key: String = (0..100).map(|_| rng.gen_range('a'..='z')).collect();
    let ciphertext = encode(&key, s).unwrap();
    (key, ciphertext)
}
