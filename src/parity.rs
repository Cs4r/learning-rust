
struct Parity {
    byte: u8,
    bit: bool,
}

impl Parity {
    pub fn new() -> Self {
        Parity {
            byte: 0,
            bit: false,
        }
    }

    pub fn get(&self) -> bool {
        self.bit
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.byte = byte;
        let mut counter = 0;

        for i in (0..8).rev() {
            if self.byte & (1 << i) != 0 {
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