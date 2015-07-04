// Smallest multiple

use primes::PrimeSet;
use std::iter;

pub fn result(max: u64) -> u64 {
    PrimeSet::new().iter()
        .take_while(|&x| x <= max)
        .map(
            |x| iter::repeat(1)
                .scan(1, |product, _| { *product = *product * x; Some(*product) })
                .take_while(|&y| y <= max)
                .last()
                .unwrap()
        )
        .fold(1, |product, x| product * x)
}

#[test]
fn projecteuler() {
    assert_eq!(232_792_560, result(20));
}
