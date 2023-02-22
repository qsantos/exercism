pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return vec![];
    }
    let mut primes = vec![2];
    for candidate in (3..=upper_bound).step_by(2) {
        if primes
            .iter()
            .take_while(|&prime| prime * prime <= candidate)
            .all(|prime| candidate % prime != 0)
        {
            primes.push(candidate);
        }
    }
    primes
}
