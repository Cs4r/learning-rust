use std::fmt::Display;

#[derive(Debug, Hash, Eq)]
pub struct BitVector {
    data: Vec<u8>,
    n_bits: usize,
}

impl BitVector {
    pub fn new() -> Self {
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

    pub fn n_bits(&self) -> usize {
        self.n_bits
    }

    pub fn n_bytes(&self) -> usize {
        n_bytes(self.n_bits)
    }

    pub fn add_bit(&mut self, bit: bool) {
        if 8 * self.data.len() == self.n_bits {
            let new_len = if self.data.len() == 0 {
                1
            } else {
                self.data.len() * 2
            };
            self.data.resize(new_len, 0);
        }

        self.n_bits += 1;

        let shift = 8 * self.n_bytes() - self.n_bits;
        let last_byte = self.n_bytes() - 1;

        if bit {
            self.data[last_byte] ^= 1 << shift;
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

fn n_bytes(n_bits: usize) -> usize {
    (n_bits + 7) >> 3
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
                write!(f, "{}" , if bit_set {'1'} else {'0'}   )?;

            }
        }

        let last_bit = 8 * total_bytes - self.n_bits;

        for bit_pos in (last_bit..=7).rev() {
            let bit_set = self.data[total_bytes-1] & (1 << bit_pos) != 0;
            write!(f, "{}" , if bit_set {'1'} else {'0'}   )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod helpers {
        use super::*;

        #[test]
        fn test_n_bytes_calculation() {
            assert_eq!(n_bytes(0), 0);
            assert_eq!(n_bytes(1), 1);
            assert_eq!(n_bytes(7), 1);
            assert_eq!(n_bytes(8), 1);
            assert_eq!(n_bytes(9), 2);
            assert_eq!(n_bytes(15), 2);
            assert_eq!(n_bytes(16), 2);
            assert_eq!(n_bytes(17), 3);
        }
    }

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

    mod add_bit_behavior {
        use super::*;

        #[test]
        fn test_add_bit_true_to_empty_vector() {
            let mut bv = BitVector::new();
            bv.add_bit(true);
            assert_eq!(bv.n_bits(), 1);
            assert_eq!(bv.n_bytes(), 1);
            assert_eq!(bv.data[0], 0b10000000);
        }

        #[test]
        fn test_add_bit_false_to_empty_vector() {
            let mut bv = BitVector::new();
            bv.add_bit(false);
            assert_eq!(bv.n_bits(), 1);
            assert_eq!(bv.n_bytes(), 1);
            assert_eq!(bv.data[0], 0b00000000);
        }

        #[test]
        fn test_add_bit_alternating_bits() {
            let mut bv = BitVector::new();
            bv.add_bit(true);
            bv.add_bit(false);
            bv.add_bit(true);
            bv.add_bit(false);
            assert_eq!(bv.n_bits(), 4);
            assert_eq!(bv.data.len(), 1);
            assert_eq!(bv.data[0], 0b10100000);
        }

        #[test]
        fn test_add_bit_fill_one_byte() {
            let mut bv = BitVector::new();
            for _ in 0..8 {
                bv.add_bit(true);
            }
            assert_eq!(bv.n_bits(), 8);
            assert_eq!(bv.data.len(), 1);
            assert_eq!(bv.data[0], 0b11111111);
        }

        #[test]
        fn test_add_bit_triggers_resize() {
            let mut bv = BitVector::new();
            for _ in 0..8 {
                bv.add_bit(false)
            }
            assert_eq!(bv.data.len(), 1);
            bv.add_bit(true); // 9th bit triggers resize
            assert_eq!(bv.n_bits(), 9);
            assert_eq!(bv.data.len(), 2);
            assert_eq!(bv.data[0], 0b00000000);
            assert_eq!(bv.data[1], 0b10000000);
        }

        #[test]
        fn test_add_bit_multiple_resizes() {
            let mut bv = BitVector::new();
            for i in 0..100 {
                bv.add_bit(i % 2 == 0);
            }
            assert_eq!(bv.n_bits(), 100);
            assert!(bv.data.len() > 13);
            assert_eq!(bv.data[0], 0b10101010);
            assert_eq!(bv.data[12], 0b10100000);
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

    mod display_behavior {
        use super::*;

        #[test]
        fn test_display_empty() {
            let bv = BitVector::new();
            let output = format!("{}", bv);
            assert_eq!(output, "");
        }

        #[test]
        fn test_display_single_bit_true() {
            let mut bv = BitVector::new();
            bv.add_bit(true);
            let output = format!("{}", bv);
            assert_eq!(output, "1");
        }

        #[test]
        fn test_display_single_bit_false() {
            let mut bv = BitVector::new();
            bv.add_bit(false);
            let output = format!("{}", bv);
            assert_eq!(output, "0");
        }

        #[test]
        fn test_display_multiple_bits() {
            let mut bv = BitVector::new();
            bv.add_bit(true);
            bv.add_bit(false);
            bv.add_bit(true);
            bv.add_bit(true);
            bv.add_bit(false);
            let output = format!("{}", bv);
            assert_eq!(output, "10110");
        }

        #[test]
        fn test_display_full_byte() {
            let mut bv = BitVector::new();
            for _ in 0..8 {
                bv.add_bit(true);
            }
            let output = format!("{}", bv);
            assert_eq!(output, "11111111");
        }

        #[test]
        fn test_display_partial_byte() {
            let mut bv = BitVector::new();
            bv.add_bit(true);
            bv.add_bit(false);
            bv.add_bit(false);
            bv.add_bit(true);
            let output = format!("{}", bv);
            assert_eq!(output, "1001");
        }

        #[test]
        fn test_display_multiple_bytes() {
            let mut bv = BitVector::new();
            // 12 bits: 10101010 1100 (last 4 bits)
            let bits = [
                true, false, true, false, true, false, true, false, // 8 bits = 0b10101010
                true, true, false, false, // 4 bits = 0b1100 (pad ignored)
            ];
            for &bit in &bits {
                bv.add_bit(bit);
            }
            let output = format!("{}", bv);
            assert_eq!(output, "101010101100");
        }
    }
}
