pub fn square(s: u32) -> u64 {
    assert!((1..=64).contains(&s), "Square must be between 1 and 64");
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    std::u64::MAX
}
