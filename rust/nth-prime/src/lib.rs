pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut primes = vec![2];
    let mut current = 1;
    while primes.len() <= (n as usize) {
        current += 2;
        while primes
            .iter()
            .take_while(|&p| p * p <= current)
            .any(|p| current % p == 0)
        {
            current += 2;
        }
        primes.push(current);
    }
    current
}
