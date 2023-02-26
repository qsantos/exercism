use std::fmt::Display;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: Display> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let mut double = false;
        let mut s = 0;
        let mut digits = 0;
        let code = format!("{}", self);
        for &c in code.as_bytes().iter().rev() {
            if c == b' ' {
                continue;
            }
            if !(b'0'..=b'9').contains(&c) {
                return false;
            }
            let mut v = c - b'0';
            if double {
                v *= 2;
                if v >= 10 {
                    v -= 9;
                }
            }
            s += v;
            double = !double;
            digits += 1;
        }
        digits >= 2 && s % 10 == 0
    }
}
