// Summation of primes

use primes::PrimeSet;

pub fn result(limit: u64) -> u64 {
    PrimeSet::new().iter()
        .take_while(|&x| x < limit)
        .fold(0, |sum, x| sum + x)
}

#[test]
fn projecteuler() {
    assert_eq!(142_913_828_922, result(2_000_000));
}
