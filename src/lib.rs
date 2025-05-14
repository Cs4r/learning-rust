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
                if self.byte & (1 << i) != 0 {  // If the i-th bit of byte is 1
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
}
