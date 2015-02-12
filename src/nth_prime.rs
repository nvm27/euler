// 10001st prime

use primes::PrimeSet;

pub fn result(position: usize) -> u64 {
    PrimeSet::new().iter()
        .nth(position - 1)
        .unwrap()
}

#[test]
fn projecteuler() {
    assert!(104_743 == result(10_001));
}
