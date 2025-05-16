pub fn parity(input: &[u8]) -> bool {
    let mut odds_count = 0;

    for byte in input {
        if byte.count_ones() % 2 == 1 {
            odds_count += 1;
        }
    }

    odds_count % 2 == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parity_even() {
        assert_eq!(parity("zxcv".as_bytes()), false);
        assert_eq!(parity("cesar".as_bytes()), false);
        assert_eq!(parity("luis".as_bytes()), false);
    }

    #[test]
    fn test_parity_odd() {
        assert_eq!(parity("1234".as_bytes()), true);
        assert_eq!(parity("abcd".as_bytes()), true);
        assert_eq!(parity("bueno".as_bytes()), true);
    }

    #[test]
    fn test_parity_empty() {
        assert_eq!(parity("".as_bytes()), false);
    }

    #[test]
    fn test_parity_single_char() {
        assert_eq!(parity("a".as_bytes()), true);
        assert_eq!(parity("b".as_bytes()), true);
    }
}