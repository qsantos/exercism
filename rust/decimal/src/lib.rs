use num_bigint::BigInt;
use std::cmp::Ordering;
use std::str::FromStr;

use std::ops::{Add, Mul, Sub};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Decimal {
    mantissa: BigInt,
    decimal_places: usize,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let digits = String::from_utf8(input.bytes().filter(|&c| c != b'.').collect()).unwrap();
        let mantissa = BigInt::from_str(&digits).unwrap();
        let decimal_places = input.bytes().rev().position(|c| c == b'.').unwrap_or(0);
        Some(
            Decimal {
                mantissa,
                decimal_places,
            }
            .normalize(),
        )
    }

    fn align(&self, rhs: &Self) -> (Self, Self) {
        match self.decimal_places.cmp(&rhs.decimal_places) {
            Ordering::Greater => {
                let d = self.decimal_places - rhs.decimal_places;
                let b = Decimal {
                    mantissa: &rhs.mantissa * BigInt::from(10).pow(d as u32),
                    decimal_places: self.decimal_places,
                };
                (self.clone(), b)
            }
            Ordering::Less => {
                let (a, b) = rhs.align(self);
                (b, a)
            }
            Ordering::Equal => (self.clone(), rhs.clone()),
        }
    }

    fn normalize(mut self) -> Self {
        while self.decimal_places > 0 && &self.mantissa % 10 == BigInt::from(0) {
            self.mantissa /= 10;
            self.decimal_places -= 1;
        }
        self
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (a, b) = self.align(other);
        a.mantissa.partial_cmp(&b.mantissa)
    }
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(self, rhs: Self) -> Self::Output {
        let (mut a, b) = self.align(&rhs);
        assert_eq!(a.decimal_places, b.decimal_places);
        a.mantissa += b.mantissa;
        a.normalize()
    }
}

impl Sub for Decimal {
    type Output = Decimal;
    fn sub(self, rhs: Self) -> Self::Output {
        let (mut a, b) = self.align(&rhs);
        assert_eq!(a.decimal_places, b.decimal_places);
        a.mantissa -= b.mantissa;
        a.normalize()
    }
}

impl Mul for Decimal {
    type Output = Decimal;
    fn mul(self, rhs: Self) -> Self::Output {
        Decimal {
            mantissa: self.mantissa * rhs.mantissa,
            decimal_places: self.decimal_places + rhs.decimal_places,
        }
        .normalize()
    }
}
