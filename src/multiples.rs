// Multiples of 3 and 5

pub fn result(size: u64) -> u64 {
    (1..size)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .fold(0, |sum, x| sum + x)
}

#[test]
fn projecteuler() {
    assert_eq!(233_168, result(1_000));
}
