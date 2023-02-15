use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    if num == 1 {
        return Some(Classification::Deficient);
    }
    let mut sum_divisors = 1;
    let mut a = 2;
    while a * a < num {
        if num % a == 0 {
            sum_divisors += a + num / a;
        }
        a += 1;
    }
    if num == a * a {
        sum_divisors += a;
    }
    Some(match sum_divisors.cmp(&num) {
        Ordering::Equal => Classification::Perfect,
        Ordering::Less => Classification::Deficient,
        Ordering::Greater => Classification::Abundant,
    })
}
