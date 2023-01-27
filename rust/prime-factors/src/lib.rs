pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut primes = vec![2];
    // factorize 2s
    while n % 2 == 0 {
        n /= 2;
        factors.push(2);
    }
    let mut current = 3;
    while n != 1 {
        // factorize
        while n % current == 0 {
            n /= current;
            factors.push(current);
        }

        // next prime
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
    factors
}
