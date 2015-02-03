pub fn result(size: u64) -> u64 {
    (1..size)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .fold(0, |sum, x| sum + x)
}
