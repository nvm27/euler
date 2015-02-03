mod multiples;
mod even_fib;

fn main() {
    println!("Multiples of 3 and 5: {}", multiples::result(1000));
    println!("Even Fibonacci numbers: {}", even_fib::result(4000000));
}
