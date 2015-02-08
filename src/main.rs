#![feature(core)]

extern crate primes;

mod multiples;
mod even_fib;
mod largest_prime;
mod palindrome;
mod smallest_multiple;

fn main() {
    println!("Multiples of 3 and 5: {}", multiples::result(1_000));
    println!("Even Fibonacci numbers: {}", even_fib::result(4_000_000));
    println!("Largest prime factor: {}", largest_prime::result(600_851_475_143));
    println!("Largest palindrome product: {}", palindrome::result(100, 1000));
    println!("Smallest multiple: {}", smallest_multiple::result(20));
}
