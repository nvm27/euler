// Sum square difference

pub fn result(begin: u64, end: u64) -> u64 {
    (begin..(end - 1))
        .flat_map(
            |x| ((x + 1)..end)
                .map(|y| x*y)
                .collect::<Vec<u64>>()
                .into_iter()
        )
        .fold(0, |sum, x| sum + x) * 2
}

#[test]
fn projecteuler() {
    assert_eq!(25_164_150, result(1, 101));
}
