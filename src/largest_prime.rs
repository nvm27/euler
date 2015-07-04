// Largest prime factor

use primes::PrimeSet;

pub fn result(number: u64) -> u64 {
    let max: u64 = (number as f64).sqrt() as u64 + 1;

    PrimeSet::new().iter()
        .take_while(|&x| x <= max)
        .filter(|&x| number % x == 0)
        .max()
        .unwrap()
}

#[test]
fn projecteuler() {
    assert_eq!(6_857, result(600_851_475_143));
}
