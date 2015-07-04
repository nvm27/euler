// Even Fibonacci numbers

use std::marker::Copy;
use std::ops::Add;

struct Fibonacci<T: Add<Output=T> + Copy> {
    first: T,
    second: T,
}

impl<T: Add<Output=T> + Copy> Iterator for Fibonacci<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let result = self.first;

        self.first = self.second;
        self.second = self.second + result;

        Some(result)
    }
}

pub fn result(max: u64) -> u64 {
    Fibonacci::<u64> { first: 1, second: 2 }
        .take_while(|&x| x <= max)
        .filter(|&x| x % 2 == 0)
        .fold(0, |sum, x| sum + x)
}

#[test]
fn projecteuler() {
    assert_eq!(4_613_732, result(4_000_000));
}
