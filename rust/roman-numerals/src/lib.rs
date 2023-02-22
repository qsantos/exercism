use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    value: String,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.value)
    }
}

const ROMAN_NUMBERS: [(&str, u32); 13] = [
    ("M", 1_000),
    ("CM", 900),
    ("D", 500),
    ("CD", 400),
    ("C", 100),
    ("XC", 90),
    ("L", 50),
    ("XL", 40),
    ("X", 10),
    ("IX", 9),
    ("V", 5),
    ("IV", 4),
    ("I", 1),
];

impl From<u32> for Roman {
    fn from(mut num: u32) -> Self {
        let mut value = String::new();
        for (s, v) in ROMAN_NUMBERS {
            while num >= v {
                value.push_str(s);
                num -= v;
            }
        }
        assert_eq!(num, 0);
        Roman { value }
    }
}
