pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut ret = 0;
    for candidate in 1..limit {
        if factors
            .iter()
            .any(|&factor| factor != 0 && candidate % factor == 0)
        {
            ret += candidate;
        }
    }
    ret
}
