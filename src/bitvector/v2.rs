use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Hash, Eq)]
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

    fn from_value(n: u32, n_bits: usize) -> BitVector {
        let mut bit_vector = BitVector::new();

        bit_vector.n_bits = 0;
        bit_vector.data.resize(n_bytes(n_bits), 0);

        let mut n = n;

        for _ in 0..n_bits {
            bit_vector.add_bit(n & 1 != 0);
            n /= 2;
        }

        bit_vector
    }

    fn n_bits(&self) -> usize {
        self.n_bits
    }

    fn n_bytes(&self) -> usize {
        n_bytes(self.n_bits)
    }

    fn get_byte(&self, index: usize) -> u8 {
        self.data[index]
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

impl PartialEq for BitVector {
    fn eq(&self, other: &Self) -> bool {
        if self.n_bits() != other.n_bits() {
            return false;
        }

        for i in 0..self.n_bytes() {
            if self.data[i] != other.data[i] {
                return false;
            }
        }

        true
    }
}

impl Display for BitVector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.n_bits == 0 {
            return Ok(());
        }

        let total_bytes = self.n_bytes();

        for i in 0..total_bytes - 1 {
            for bit_pos in (0..=7).rev() {
                let bit_set = self.data[i] & (1 << bit_pos) != 0;
                write!(f, "{}", if bit_set { '1' } else { '0' })?;
            }
        }

        let last_bit = 8 * total_bytes - self.n_bits;

        for bit_pos in (last_bit..=7).rev() {
            let bit_set = self.data[total_bytes - 1] & (1 << bit_pos) != 0;
            write!(f, "{}", if bit_set { '1' } else { '0' })?;
        }

        Ok(())
    }
}

impl Debug for BitVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BitVector({})", self)
    }
}

impl FromStr for BitVector {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BitVector::from_str(s)
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
            assert_eq!(bv.n_bytes(), expected_bytes);

            assert!(bv.data.iter().all(|&b| b == 0));
        }
    }

    mod set_and_get_behavior {
        use super::*;

        #[test]
        fn test_set_and_get_single_bit() {
            let mut bv = BitVector::with_bits(8);

            for i in 0..8 {
                assert_eq!(bv.get(i), false, "bit {} should be false initially", i);
            }

            bv.set(3, true);
            assert_eq!(
                bv.get(3),
                true,
                "bit 3 should be true after setting to true"
            );

            bv.set(3, false);
            assert_eq!(
                bv.get(3),
                false,
                "bit 3 should be false after setting to false"
            );
        }

        #[test]
        fn test_set_multiple_bits() {
            let mut bv = BitVector::with_bits(16);

            bv.set(0, true);
            bv.set(7, true);
            bv.set(15, true);

            assert!(bv.get(0));
            assert!(bv.get(7));
            assert!(bv.get(15));

            assert!(!bv.get(1));
            assert!(!bv.get(8));
        }

        #[test]
        fn test_toggle_bit() {
            let mut bv = BitVector::with_bits(1);
            assert_eq!(bv.get(0), false);

            bv.set(0, true);
            assert_eq!(bv.get(0), true);

            bv.set(0, true);
            assert_eq!(bv.get(0), true);

            bv.set(0, false);
            assert_eq!(bv.get(0), false);
        }
    }

    mod add_bit_behavior {
        use super::*;

        #[test]
        fn test_add_bit_true_to_empty_vector() {
            let mut bv = BitVector::new();
            bv.add_bit(true);
            assert_eq!(bv.n_bits(), 1);
            assert_eq!(bv.n_bytes(), 1);

            assert_eq!(bv.get_byte(0), 0b00000001);
        }

        #[test]
        fn test_add_bit_false_to_empty_vector() {
            let mut bv = BitVector::new();
            bv.add_bit(false);
            assert_eq!(bv.n_bits(), 1);
            assert_eq!(bv.n_bytes(), 1);
            assert_eq!(bv.get_byte(0), 0b00000000);
        }

        #[test]
        fn test_add_bit_alternating_bits() {
            let mut bv = BitVector::new();
            bv.add_bit(true);
            bv.add_bit(false);
            bv.add_bit(true);
            bv.add_bit(false);
            assert_eq!(bv.n_bits(), 4);
            assert_eq!(bv.n_bytes(), 1);
            assert_eq!(bv.get_byte(0), 0b00000101);
        }

        #[test]
        fn test_add_bit_fill_one_byte() {
            let mut bv = BitVector::new();

            for _ in 0..8 {
                bv.add_bit(true);
            }
            assert_eq!(bv.n_bits(), 8);
            assert_eq!(bv.n_bytes(), 1);
            assert_eq!(bv.get_byte(0), 0b11111111);
        }

        #[test]
        fn test_add_bit_triggers_resize() {
            let mut bv = BitVector::new();

            for _ in 0..8 {
                bv.add_bit(false)
            }

            assert_eq!(bv.n_bytes(), 1);

            bv.add_bit(true); // 9th bit triggers resize

            assert_eq!(bv.n_bits(), 9);
            assert_eq!(bv.n_bytes(), 2);
            assert_eq!(bv.get_byte(0), 0b00000000);
            assert_eq!(bv.get_byte(1), 0b00000001);
        }

        #[test]
        fn test_add_bit_multiple_resizes() {
            let mut bv = BitVector::new();

            for i in 0..100 {
                bv.add_bit(i % 2 == 0);
            }

            assert_eq!(bv.n_bits(), 100);
            assert_eq!(bv.n_bytes(), 13);

            assert_eq!(bv.get_byte(0), 0b01010101);
            assert_eq!(bv.get_byte(12), 0b00000101);
        }

        #[test]
        fn test_add_bit_only_false() {
            let mut bv = BitVector::new();
            for _ in 0..16 {
                bv.add_bit(false);
            }
            assert_eq!(bv.n_bits(), 16);
            assert_eq!(bv.data, vec![0b00000000, 0b00000000]);
        }

        #[test]
        fn test_add_bit_only_true() {
            let mut bv = BitVector::new();

            for _ in 0..16 {
                bv.add_bit(true);
            }

            assert_eq!(bv.n_bits(), 16);
            assert_eq!(bv.data, vec![0b11111111, 0b11111111]);
        }
    }

    mod from_value_behavior {
        use super::*;

        #[test]
        fn test_from_value_zero_bits() {
            let bv = BitVector::from_value(0, 0);
            assert_eq!(bv.n_bits(), 0);
            assert_eq!(bv.n_bytes(), 0);
        }

        #[test]
        fn test_from_value_one_bit_set() {
            let bv = BitVector::from_value(1, 1);
            assert_eq!(bv.n_bits(), 1);
            assert_eq!(bv.get(0), true);
        }

        #[test]
        fn test_from_value_multiple_bits() {
            let bv = BitVector::from_value(5, 4);
            assert_eq!(bv.n_bits(), 4);
            assert_eq!(bv.get(0), true); // LSB
            assert_eq!(bv.get(1), false);
            assert_eq!(bv.get(2), true);
            assert_eq!(bv.get(3), false);
        }

        #[test]
        fn test_from_value_padded_with_zeros() {
            let bv = BitVector::from_value(3, 8);
            assert_eq!(bv.n_bits(), 8);
            assert_eq!(bv.get(0), true);
            assert_eq!(bv.get(1), true);

            for i in 2..8 {
                assert_eq!(bv.get(i), false, "bit {} should be false", i);
            }
        }

        #[test]
        fn test_from_value_truncates_higher_bits() {
            let bv = BitVector::from_value(255, 4);
            assert_eq!(bv.n_bits(), 4);
            for i in 0..4 {
                assert!(bv.get(i), "bit {} should be true", i);
            }
        }
    }

    mod equality_behavior {
        use super::*;

        #[test]
        fn eq_same_empty_vectors() {
            let bv1 = BitVector::new();
            let bv2 = BitVector::new();
            assert_eq!(bv1, bv2);
        }

        #[test]
        fn eq_same_content() {
            let bv1 = BitVector::with_bits(10);
            let bv2 = BitVector::with_bits(10);

            assert_eq!(bv1, bv2);
        }

        #[test]
        fn neq_different_n_bits() {
            let bv1 = BitVector::with_bits(5);
            let bv2 = BitVector::with_bits(10);
            assert_ne!(bv1, bv2);
        }

        #[test]
        fn neq_different_data() {
            let mut bv1 = BitVector::with_bits(8);
            let mut bv2 = BitVector::with_bits(8);

            bv1.data[0] = 0b0000_0001;
            bv2.data[0] = 0b0000_0010;

            assert_ne!(bv1, bv2);
        }

        #[test]
        fn test_not_equal_due_to_used_bits_difference() {
            let mut bv1 = BitVector::with_bits(10);
            let mut bv2 = BitVector::with_bits(10);

            bv1.data[0] = 0b10101010;
            bv2.data[0] = 0b10101010;

            bv1.data[1] = 0b00000011;
            bv2.data[1] = 0b00000010; // difference in used bit

            assert_ne!(bv1, bv2);
        }

        #[test]
        fn test_equal_full_bytes() {
            let mut bv1 = BitVector::with_bits(16);
            let mut bv2 = BitVector::with_bits(16);

            bv1.data[0] = 0xFF;
            bv1.data[1] = 0xAA;

            bv2.data[0] = 0xFF;
            bv2.data[1] = 0xAA;

            assert_eq!(bv1, bv2);
        }

        #[test]
        fn test_not_equal_different_sizes() {
            let bv1 = BitVector::with_bits(8);
            let bv2 = BitVector::with_bits(9);

            assert_ne!(bv1, bv2);
        }

        #[test]
        fn test_eq_zero_bits_nonempty_data() {
            let mut bv1 = BitVector::with_bits(0);
            let mut bv2 = BitVector::with_bits(0);

            bv1.data = vec![0b10101010];
            bv2.data = vec![0b11110000];

            assert_eq!(bv1, bv2);
        }
    }

    mod hash_behavior {
        use super::*;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        fn calculate_hash<T: Hash>(t: &T) -> u64 {
            let mut hasher = DefaultHasher::new();
            t.hash(&mut hasher);
            hasher.finish()
        }

        #[test]
        fn test_equal_vectors_have_same_hash() {
            let mut a = BitVector::new();
            let mut b = BitVector::new();

            a.add_bit(true);
            a.add_bit(false);
            a.add_bit(true);

            b.add_bit(true);
            b.add_bit(false);
            b.add_bit(true);

            assert_eq!(a, b);
            assert_eq!(calculate_hash(&a), calculate_hash(&b));
        }

        #[test]
        fn test_different_vectors_have_different_hashes() {
            let mut a = BitVector::new();
            let mut b = BitVector::new();

            a.add_bit(true);
            a.add_bit(false);
            a.add_bit(true);

            b.add_bit(true);
            b.add_bit(true);
            b.add_bit(false);

            assert_ne!(a, b);
            assert_ne!(calculate_hash(&a), calculate_hash(&b));
        }
    }
}
