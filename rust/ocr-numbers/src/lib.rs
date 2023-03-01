// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

const DIGITS: [&[u8; 30]; 4] = [
    b" _     _  _     _  _  _  _  _ ",
    b"| |  | _| _||_||_ |_   ||_||_|",
    b"|_|  ||_  _|  | _||_|  ||_| _|",
    b"                              ",
];

fn identify_digit(pattern: &[&[u8]]) -> Option<usize> {
    (0..=9).find(|digit| {
        DIGITS
            .iter()
            .copied()
            .zip(pattern.iter().copied())
            .all(|(a, b)| (&a[(digit * 3)..((digit + 1) * 3)]) == b)
    })
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let rows = lines.len();
    if rows == 0 {
        return Ok(String::new());
    }
    if rows % 4 != 0 {
        return Err(Error::InvalidRowCount(rows));
    }
    let cols = lines[0].len();
    if cols % 3 != 0 {
        return Err(Error::InvalidColumnCount(cols));
    }
    assert!(lines.iter().all(|line| line.len() == cols));

    let mut ret = String::new();
    for rows in lines.chunks(4) {
        if !ret.is_empty() {
            ret.push(',');
        }
        for offset in (0..cols).step_by(3) {
            let digit_rows: Vec<&[u8]> =
                rows.iter().map(|&row| &row[offset..(offset + 3)]).collect();
            if let Some(digit) = identify_digit(&digit_rows) {
                ret.push((b'0' + digit as u8) as char);
            } else {
                ret.push('?');
            }
        }
    }
    Ok(ret)
}
