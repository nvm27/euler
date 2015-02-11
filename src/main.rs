#![feature(core)]

extern crate primes;

mod multiples;
mod even_fib;
mod largest_prime;
mod palindrome;
mod smallest_multiple;
mod square_diff;
mod nth_prime;

fn main() {
    println!("Multiples of 3 and 5: {}", multiples::result(1_000));
    println!("Even Fibonacci numbers: {}", even_fib::result(4_000_000));
    println!("Largest prime factor: {}", largest_prime::result(600_851_475_143));
    println!("Largest palindrome product: {}", palindrome::result(100, 1000));
    println!("Smallest multiple: {}", smallest_multiple::result(20));
    println!("Sum square difference: {}", square_diff::result(1, 101));
    println!("10001st prime: {}", nth_prime::result(10_001));
}
