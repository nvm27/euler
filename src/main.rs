extern crate primes;

mod multiples;
mod even_fib;
mod largest_prime;

fn main() {
    println!("Multiples of 3 and 5: {}", multiples::result(1_000));
    println!("Even Fibonacci numbers: {}", even_fib::result(4_000_000));
    println!("Largest prime factor: {}", largest_prime::result(600_851_475_143));
}
