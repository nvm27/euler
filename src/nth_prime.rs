use primes::PrimeSet;

pub fn result(position: usize) -> u64 {
    PrimeSet::new().iter()
        .nth(position - 1)
        .unwrap()
}
