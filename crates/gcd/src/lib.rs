pub fn gcd(x: u32, y: u32) -> u32 {
    if x == 0 {
        y
    } else {
        gcd(y % x, x)
    }
}

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn test_hand() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, 1), 1);
        assert_eq!(gcd(0, 2), 2);
        assert_eq!(gcd(0, 3), 3);

        assert_eq!(gcd(1, 0), 1);
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(1, 2), 1);
        assert_eq!(gcd(1, 3), 1);

        assert_eq!(gcd(2, 0), 2);
        assert_eq!(gcd(2, 1), 1);
        assert_eq!(gcd(2, 2), 2);
        assert_eq!(gcd(2, 3), 1);
        assert_eq!(gcd(2, 4), 2);

        assert_eq!(gcd(3, 0), 3);
        assert_eq!(gcd(3, 1), 1);
        assert_eq!(gcd(3, 2), 1);
        assert_eq!(gcd(3, 3), 3);
        assert_eq!(gcd(3, 4), 1);

        assert_eq!(gcd(6, 0), 6);
        assert_eq!(gcd(6, 1), 1);
        assert_eq!(gcd(6, 2), 2);
        assert_eq!(gcd(6, 3), 3);
        assert_eq!(gcd(6, 4), 2);
        assert_eq!(gcd(6, 5), 1);
        assert_eq!(gcd(6, 6), 6);
        assert_eq!(gcd(6, 7), 1);
        assert_eq!(gcd(6, 8), 2);
        assert_eq!(gcd(6, 9), 3);
        assert_eq!(gcd(6, 10), 2);
        assert_eq!(gcd(6, 11), 1);
        assert_eq!(gcd(6, 12), 6);
    }
}
