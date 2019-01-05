pub fn collatz_len(mut n: u32) -> usize {
    let mut count = 0;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        count += 1;
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        assert_eq!(collatz_len(1), 0);
        assert_eq!(collatz_len(2), 1);
        assert_eq!(collatz_len(3), 7);
        assert_eq!(collatz_len(4), 2);
        assert_eq!(collatz_len(5), 5);
    }
}
