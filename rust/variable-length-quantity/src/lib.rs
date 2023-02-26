#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut ret = Vec::new();
    for mut value in values.iter().rev().copied() {
        ret.push((value % 128) as u8);
        value /= 128;
        while value > 0 {
            ret.push(0x80 | (value % 128) as u8);
            value /= 128;
        }
    }
    ret.reverse();
    ret
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut ret = Vec::new();
    let mut value = 0u32;
    let mut in_sequence = false;
    for byte in bytes.iter().copied() {
        value = value.checked_mul(128).ok_or(Error::Overflow)?;
        value |= (byte & 0x7f) as u32;
        in_sequence = true;
        if byte & 0x80 == 0 {
            ret.push(value);
            value = 0;
            in_sequence = false;
        }
    }
    if in_sequence {
        return Err(Error::IncompleteNumber);
    }
    Ok(ret)
}
