/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let digits: Vec<usize> = isbn.chars().filter_map(|c| "0123456789X".find(c)).collect();
    if digits.len() != 10 {
        return false;
    }
    if digits[..9].contains(&10) {
        return false;
    }
    digits
        .iter()
        .enumerate()
        .map(|(i, d)| d * (10 - i))
        .sum::<usize>()
        % 11
        == 0
}
