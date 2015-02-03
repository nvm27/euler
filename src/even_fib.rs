use std::num::Int;

struct Fibonacci<T: Int> {
    first: T,
    second: T,
}

impl<T: Int> Iterator for Fibonacci<T> {
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
