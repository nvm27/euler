use primes::PrimeSet;
use std::iter;

pub fn result(max: u64) -> u64 {
    PrimeSet::new().iter()
        .take_while(|&x| x <= max)
        .map(
            |x| iter::iterate(1, |y| x*y)
                .take_while(|&y| y <= max)
                .last()
                .unwrap()
        )
        .fold(1, |product, x| product * x)
}

#[test]
fn projecteuler() {
    assert!(232_792_560 == result(20));
}
