// Special Pythagorean triplet

pub fn result(sum: i64) -> Vec<i64> {
    let b_vec = (1..sum/2)
        .filter(|&x| x % 2 != 0)
        .filter(|&x| sum*(x - sum/2) % (x - sum) == 0)
        .collect::<Vec<i64>>();

    let a_vec = b_vec.iter()
        .map(|&x| sum*(x - sum/2)/(x - sum))
        .collect::<Vec<i64>>();

    let c_vec = a_vec.iter()
        .zip(b_vec.iter())
        .map(|(&a, &b)| sum - (a + b))
        .collect::<Vec<i64>>();

    a_vec.iter()
        .zip(b_vec.iter())
        .zip(c_vec.iter())
        .map(|((&a, &b), &c)| a * b * c)
        .collect::<Vec<i64>>()
}

#[test]
fn projecteuler() {
    let result = result(1_000);

    assert_eq!(1, result.len());
    assert_eq!(31_875_000, result[0]);
}
