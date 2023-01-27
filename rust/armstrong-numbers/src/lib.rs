pub fn is_armstrong_number(num: u64) -> bool {
    let decimal = num.to_string();
    let length = decimal.len();
    let sum: u64 = decimal
        .bytes()
        .map(|c| ((c - b'0') as u64).pow(length as u32))
        .sum();
    sum == num
}
