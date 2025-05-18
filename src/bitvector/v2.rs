struct BitVector {
    data: Vec<u8>,
    n_bits: usize,
}

impl BitVector {
    pub fn new() -> BitVector {
        Self::with_bits(0)
    }

    fn with_bits(n_bits: usize) -> BitVector {
        if n_bits == 0 {
            BitVector {
                data: vec![],
                n_bits: 0,
            }
        } else {
            let n_bytes = n_bytes(n_bits);
            let vec: Vec<u8> = vec![0; n_bytes];

            BitVector { data: vec, n_bits }
        }
    }

    fn n_bits(&self) -> usize {
        self.n_bits
    }

    fn n_bytes(&self) -> usize {
        n_bytes(self.n_bits())
    }

    fn get(&self, bit_n: usize) -> bool {
        self.data[bit_n >> 3] & (1 << (bit_n & 7)) != 0
    }

    fn set(&mut self, bit_n: usize, value: bool) {
        if value {
            if !self.get(bit_n) {
                self.data[bit_n >> 3] = self.data[bit_n >> 3] ^ (1 << (bit_n & 7))
            }
        } else {
            if self.get(bit_n) {
                self.data[bit_n >> 3] = self.data[bit_n >> 3] ^ (1 << (bit_n & 7));
            }
        }
    }

    fn add_bit(&mut self, value: bool) {
        if 8 * self.data.len() == self.n_bits {
            let new_len = if self.data.len() == 0 {
                1
            } else {
                self.data.len() * 2
            };
            self.data.resize(new_len, 0);
        }

        self.n_bits += 1;

        if value {
            self.set(self.n_bits - 1, true);
        }
    }
}

fn n_bytes(n_bits: usize) -> usize {
    (n_bits + 7) >> 3
}

#[cfg(test)]
mod tests {
    use super::*;

    mod constructors {
        use super::*;

        #[test]
        fn test_new_creates_empty_vector() {
            let bv = BitVector::new();
            assert_eq!(bv.n_bits(), 0);
            assert_eq!(bv.n_bytes(), 0);
            assert!(bv.data.is_empty());
        }

        #[test]
        fn test_with_bits_zero_creates_empty_vector() {
            let bv = BitVector::with_bits(0);
            assert_eq!(bv.n_bits(), 0);
            assert_eq!(bv.n_bytes(), 0);
            assert!(bv.data.is_empty());
        }

        #[test]
        fn test_with_bits_nonzero_creates_correct_size() {
            let bits = 13;
            let bv = BitVector::with_bits(bits);
            assert_eq!(bv.n_bits(), bits);

            let expected_bytes = n_bytes(bits);
            assert_eq!(bv.n_bytes(), expected_bytes);
            assert_eq!(bv.data.len(), expected_bytes);

            assert!(bv.data.iter().all(|&b| b == 0));
        }
    }
}
