#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    } else if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    for c in string_digits.chars() {
        if !c.is_ascii_digit() {
            return Err(Error::InvalidDigit(c));
        }
    }
    Ok(string_digits
        .as_bytes()
        .windows(span)
        .map(|chunk| chunk.iter().map(|digit| (digit - b'0') as u64).product())
        .max()
        .unwrap())
}
