pub mod parity {

    struct Parity {
        byte: u8,
        bit: bool,
    }

    impl Parity {
        fn new() -> Self {
            Parity {
                byte: 0,
                bit: false,
            }
        }

        fn get(&self) -> bool {
            self.bit
        }

        fn add_byte(&mut self, byte: u8) {
            self.byte = byte;
            let mut counter = 0;

            for i in (0..8).rev() {
                if self.byte & (1 << i) != 0 {
                    // If the i-th bit of byte is 1
                    counter += 1;
                }
            }

            self.bit = counter % 2 == 1;
        }
    }

    pub fn parity(input: &str) -> bool {
        let mut odds = 0;

        for c in input.chars() {
            let mut parity = Parity::new();
            parity.add_byte(c as u8);

            if parity.get() {
                odds += 1;
            }
        }

        odds % 2 == 1
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parity_even() {
            assert_eq!(parity("zxcv"), false);
            assert_eq!(parity("cesar"), false);
            assert_eq!(parity("luis"), false);
        }

        #[test]
        fn test_parity_odd() {
            assert_eq!(parity("1234"), true);
            assert_eq!(parity("abcd"), true);
            assert_eq!(parity("mari"), true);
        }

        #[test]
        fn test_parity_empty() {
            assert_eq!(parity(""), false);
        }

        #[test]
        fn test_parity_single_char() {
            assert_eq!(parity("a"), true);
            assert_eq!(parity("b"), true);
        }
    }
}

pub mod bits {

    pub fn str_to_bits(input: &str) -> String {
        let mut bytes = String::new();

        for (i, c) in input.chars().enumerate() {
            let byte = char_to_bits(c as u8);
            bytes.push_str(&byte);
            if i != input.len() - 1 {
                bytes.push(' ');
            }
        }

        bytes
    }

    fn char_to_bits(byte: u8) -> String {
        let mut bits = String::new();

        for i in (0..8).rev() {
            if byte & (1 << i) != 0 {
                bits.push('1');
            } else {
                bits.push('0');
            }
        }

        bits
    }
}

