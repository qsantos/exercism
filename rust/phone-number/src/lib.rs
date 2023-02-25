pub fn number(user_number: &str) -> Option<String> {
    let mut digits: Vec<char> = user_number.chars().filter(|c| c.is_numeric()).collect();
    if digits[0] == '1' {
        digits.remove(0);
    }
    if digits.len() == 10 && ('2'..='9').contains(&digits[0]) && ('2'..='9').contains(&digits[3]) {
        Some(digits.iter().collect())
    } else {
        None
    }
}
