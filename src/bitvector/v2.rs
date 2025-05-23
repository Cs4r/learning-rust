use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Hash, Eq)]
pub struct BitVector {
    data: Vec<u8>,
    n_bits: usize,
}

impl BitVector {
    pub fn new() -> BitVector {
        Self::with_bits(0)
    }

    pub fn with_bits(n_bits: usize) -> BitVector {
        let n_bytes = n_bytes(n_bits);
        let vec: Vec<u8> = vec![0; n_bytes];

        BitVector { data: vec, n_bits }
    }

    pub fn from_value(n: u32, n_bits: usize) -> BitVector {
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
    

    pub fn n_bits(&self) -> usize {
        self.n_bits
    }

    pub fn n_bytes(&self) -> usize {
        n_bytes(self.n_bits)
    }

    pub fn get_byte(&self, index: usize) -> u8 {
        self.data[index]
    }

    pub fn get(&self, bit_n: usize) -> bool {
        self.data[bit_n >> 3] & (1 << (bit_n & 7)) != 0
    }

    pub fn set(&mut self, bit_n: usize, value: bool) {
        if value {
            if !self.get(bit_n) {
                self.data[bit_n >> 3] ^= 1 << (bit_n & 7);
            }
        } else {
            if self.get(bit_n) {
                self.data[bit_n >> 3] ^= 1 << (bit_n & 7);
            }
        }
    }

    pub fn add_bit(&mut self, value: bool) {
        if 8 * self.data.len() == self.n_bits {
            // vector is full
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

    pub fn from_str(s: &str) -> Result<Self, String> {
        let mut bv = BitVector::new();

        for (i, c) in s.chars().enumerate() {
            match c {
                '0' => bv.add_bit(false),
                '1' => bv.add_bit(true),
                other => return Err(format!("Invalid character '{}' at position {}", other, i)),
            }
        }

        Ok(bv)
    }

    pub fn revert(&mut self) {
        for i in 0..(self.n_bits / 2) {
            let j = self.n_bits - 1 - i;

            let x = self.get(i);
            let y = self.get(j);

            self.set(i, y);
            self.set(j, x);
        }
    }

    pub fn append(&mut self, other: BitVector) {
        if other.n_bits == 0 {
            return;
        }

        if self.n_bits == 0 {
            *self = other;
        } else {
            for i in 0..other.n_bits {
                self.add_bit(other.get(i));
            }
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
        for i in 0..self.n_bits {
            write!(f, "{}", if self.get(i) { '1' } else { '0' })?;
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

impl Clone for BitVector {
    fn clone(&self) -> Self {
        BitVector {
            data: self.data.clone(),
            n_bits: self.n_bits,
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

            println!("{:08b}", bv.data[0]);
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

    mod debug_behavior {
        use super::*;

        #[test]
        fn debug_shows_bitvector_with_label() {
            let mut bv = BitVector::new();

            bv.add_bit(true);
            bv.add_bit(false);
            bv.add_bit(true);
            bv.add_bit(false);
            bv.add_bit(true);

            let debug_output = format!("{:?}", bv);
            assert_eq!(debug_output, "BitVector(10101)");
        }

        #[test]
        fn debug_empty_bitvector() {
            let bv = BitVector::new();
            let debug_output = format!("{:?}", bv);
            assert_eq!(debug_output, "BitVector()");
        }

        #[test]
        fn debug_long_bitvector() {
            let mut bv = BitVector::new();

            for char in "1100101001110001".chars() {
                if char == '1' {
                    bv.add_bit(true);
                } else {
                    bv.add_bit(false);
                }
            }

            let debug_output = format!("{:?}", bv);
            assert_eq!(debug_output, "BitVector(1100101001110001)");
        }
    }

    mod from_str_behavior {
        use super::*;

        #[test]
        fn parses_binary_string_correctly() {
            let bv = BitVector::from_str("10101").unwrap();
            assert_eq!(bv.n_bits(), 5);

            let bit_string = format!("{}", &bv);
            assert_eq!(bit_string, "10101");
        }

        #[test]
        fn handles_empty_string() {
            let bv = BitVector::from_str("").unwrap();
            assert_eq!(bv.n_bits(), 0);
            assert_eq!(bv.n_bytes(), 0);
        }

        #[test]
        fn fails_on_invalid_character() {
            let err = BitVector::from_str("10a01").unwrap_err();
            assert_eq!(err, "Invalid character 'a' at position 2");
        }

        #[test]
        fn parses_long_binary_string() {
            let input = "1100101001110001110101010101010111101"; // 37 bits
            let bv = BitVector::from_str(input).unwrap();
            assert_eq!(bv.n_bits(), input.len());

            let reconstructed = format!("{}", &bv);
            assert_eq!(reconstructed, input);
        }

        #[test]
        fn parses_using_parse_method() {
            let bv: BitVector = "11010".parse().unwrap();
            assert_eq!(bv.n_bits(), 5);

            let reconstructed = format!("{}", &bv);
            assert_eq!(reconstructed, "11010");
        }
    }

    mod revert_behavior {
        use super::*;
        use std::hint::assert_unchecked;

        #[test]
        fn test_revert_empty_vector() {
            let mut bv1 = BitVector::new();
            let expected = bv1.clone();

            bv1.revert();

            assert_eq!(bv1, expected);
        }

        #[test]
        fn test_revert_with_one_bit_vector() {
            let mut bv1: BitVector = "1".parse().unwrap();
            bv1.revert();

            let expected = "1".parse().unwrap();
            assert_eq!(bv1, expected);
        }

        #[test]
        fn test_revert_with_two_bit_vectors() {
            let mut bv1: BitVector = "10".parse().unwrap();
            bv1.revert();

            let expected = "01".parse().unwrap();
            assert_eq!(bv1, expected);
        }

        #[test]
        fn test_revert_with_one_byte() {
            let mut bv1: BitVector = "10101010".parse().unwrap();
            bv1.revert();

            let expected = "01010101".parse().unwrap();
            assert_eq!(bv1, expected);
        }

        #[test]
        fn test_revert_with_more_than_8_bits() {
            let mut bv1: BitVector = "100100000".parse().unwrap();
            bv1.revert();

            let expected = "000001001".parse().unwrap();
            assert_eq!(bv1, expected);
        }

        #[test]
        fn test_revert_with_2_bytes() {
            let mut bv1: BitVector = "1110000100010110".parse().unwrap();
            bv1.revert();

            let expected = "0110100010000111".parse().unwrap();
            assert_eq!(bv1, expected);
        }
    }

    mod append_behavior {

        use super::*;

        #[test]
        fn test_append_empty_vector_to_empty_vector() {
            let mut bv = BitVector::new();
            bv.append(BitVector::new());

            assert_eq!(bv.data, vec![]);
            assert_eq!(bv.n_bits, 0);
        }

        #[test]
        fn test_append_vector_to_empty_vector() {
            let mut bv = BitVector::new();
            const VALUE: &str = "10101";

            bv.append(VALUE.parse().unwrap());
            assert_eq!(bv.to_string(), VALUE);
        }

        #[test]
        fn test_append_empty_vector_to_not_empty_vector() {
            let mut bv : BitVector = "1111".parse().unwrap();

            bv.append(BitVector::new());

            assert_eq!(bv.to_string(), "1111");
        }

        #[test]
        fn test_append_non_empty_vector_to_not_empty_vector() {
            let mut bv : BitVector = "1111".parse().unwrap();
            let bv2 : BitVector = "0000".parse().unwrap();

            bv.append(bv2);

            assert_eq!(bv.to_string(), "11110000");
        }

        #[test]
        fn test_append_multiple_vectors_in_sequence() {
            let mut bv: BitVector = "1".parse().unwrap();
            bv.append("01".parse().unwrap());
            bv.append("10".parse().unwrap());

            assert_eq!(bv.to_string(), "10110");
        }

        #[test]
        fn test_append_to_self_with_clone() {
            let mut bv: BitVector = "101".parse().unwrap();
            let clone = bv.clone();
            bv.append(clone);

            assert_eq!(bv.to_string(), "101101");
        }

        #[test]
        fn test_append_vectors_with_non_byte_aligned_lengths() {
            let mut bv: BitVector = "101".parse().unwrap();
            let bv2: BitVector = "11".parse().unwrap();

            bv.append(bv2);
            assert_eq!(bv.to_string(), "10111");
        }

        #[test]
        fn test_data_allocation_growth() {
            let mut bv: BitVector = BitVector::new();

            for _ in 0..20 {
                bv.append("1".parse().unwrap());
            }

            assert_eq!(bv.n_bits(), 20);
            assert_eq!(bv.to_string(), "1".repeat(20));
        }

    }
}
