use std::fmt::Display;

pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let mut double = false;
        let mut s = 0;
        let mut digits = 0;
        for &c in self.code.as_bytes().iter().rev() {
            if c == b' ' {
                continue;
            }
            if !c.is_ascii_digit() {
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

impl<T: Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn {
            code: format!("{}", input),
        }
    }
}
