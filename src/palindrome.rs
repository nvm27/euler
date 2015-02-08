fn is_palindrome(number: u64) -> bool {
    let mut rev: u64 = 0;
    let mut temp: u64 = number;

    while temp > 0 {
        let digit: u64 = temp % 10;
        rev = 10 * rev + digit;
        temp = temp / 10;
    }

    number == rev
}

pub fn result(begin: u64, end: u64) -> u64 {
    (begin..end)
        .flat_map(
            |x| (begin..end)
                .map(|y| x*y)
                .collect::<Vec<u64>>()
                .into_iter()
        )
        .filter(|&x| is_palindrome(x))
        .max()
        .unwrap()
}
