pub fn lrc(input: &[u8]) -> u8 {
    let mut lrc = 0;

    for i in 0..input.len() {
        lrc  = lrc ^ input[i];
    }

    lrc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lrc_abc() {
        let expected =  65 ^ 66 ^ 67;
        assert_eq!(lrc(b"ABC"), expected); //64
    }

    #[test]
    fn test_lrc_single_char() {
        assert_eq!(lrc(b"X"), 88);
    }

    #[test]
    fn test_lrc_hello() {
        let expected = 104 ^ 101 ^ 108 ^ 108 ^ 111;
        assert_eq!(lrc(b"hello"), expected); // 98
    }

    #[test]
    fn test_lrc_symbols() {
        assert_eq!(lrc(b"!@#"), 33 ^ 64 ^ 35); // 66
    }

    #[test]
    fn test_lrc_numbers() {
        assert_eq!(lrc(b"12345"), 49 ^ 50 ^ 51 ^ 52 ^ 53); // 49
    }

    #[test]
    fn test_lrc_extended_ascii() {
        assert_eq!(lrc(b"\x80\x81\x82"), 128 ^ 129 ^ 130); // 131
    }
}
