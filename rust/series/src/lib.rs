pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        (0..=digits.len()).map(|_| String::from("")).collect()
    } else {
        let v: Vec<char> = digits.chars().collect();
        v.windows(len).map(|c| c.iter().collect()).collect()
    }
}
