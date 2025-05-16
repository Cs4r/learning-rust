use std::ops::{Add, AddAssign};


#[derive(Debug)]
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

    pub fn get(&self, bit_n: usize) -> bool {
        assert!(bit_n < self.n_bits, "Index out of bounds");

        let byte_index = byte_index(bit_n);
        let bit_mask = mask(bit_n);

        (self.data[byte_index] & bit_mask) != 0
    }

    pub fn set(&mut self, bit_n: usize, value: bool) {
        let byte_index = byte_index(bit_n);
        let bit_mask = mask(bit_n);

        if value {
            self.data[byte_index] |= bit_mask;
        } else {
            self.data[byte_index] &= !bit_mask;
        }
    }
}

impl AddAssign<bool> for BitVector {
    fn add_assign(&mut self, x: bool) {

        if 8 * self.data.len() == self.n_bits {
            let new_len = if self.data.len() == 0 { 1 } else { self.data.len() * 2 };
            self.data.resize(new_len, 0);
        }

        self.n_bits += 1;

        let shift = 8 * self.n_bytes() - self.n_bits();
        let last_byte = self.n_bytes() - 1;

        if x {
            self.data[last_byte] |= 1 << shift;
        }
    }
}

impl PartialEq for BitVector {
    fn eq(&self, other: &Self) -> bool {
        if self.n_bits != other.n_bits {
            return false;
        }

        let full_bytes = self.n_bits / 8;
        let leftover_bits = self.n_bits % 8;

        // Compare full bytes
        if self.data[..full_bytes] != other.data[..full_bytes] {
            return false;
        }

        // If there are leftover bits, compare those with a mask
        if leftover_bits > 0 {
            let mask = (1 << leftover_bits) - 1;
            let self_last = self.data[full_bytes] & mask;
            let other_last = other.data[full_bytes] & mask;
            if self_last != other_last {
                return false;
            }
        }

        true
    }
}

impl Eq for BitVector {}

fn n_bytes(n_bits: usize) -> usize {
    (n_bits + 7) >> 3
}

fn byte_index(bit_n: usize) -> usize {
    bit_n >> 3
}

fn mask(bit_n: usize) -> u8 {
    1 << (bit_n & 7)
}

#[cfg(test)]
mod tests {
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


    #[test]
    fn test_add_assign_true_to_empty_vector() {
        let mut bv = BitVector::new();
        bv += true;
        assert_eq!(bv.n_bits(), 1);
        assert_eq!(bv.n_bytes(), 1);
        assert_eq!(bv.data[0], 0b10000000);
    }

    #[test]
    fn test_add_assign_false_to_empty_vector() {
        let mut bv = BitVector::new();
        bv += false;
        assert_eq!(bv.n_bits(), 1);
        assert_eq!(bv.n_bytes(), 1);
        assert_eq!(bv.data[0], 0b00000000);
    }

    #[test]
    fn test_add_assign_alternating_bits() {
        let mut bv = BitVector::new();
        bv += true;
        bv += false;
        bv += true;
        bv += false;
        assert_eq!(bv.n_bits(), 4);
        assert_eq!(bv.data.len(), 1);
        assert_eq!(bv.data[0], 0b10100000);
    }

    #[test]
    fn test_add_assign_fill_one_byte() {
        let mut bv = BitVector::new();
        for _ in 0..8 {
            bv += true;
        }
        assert_eq!(bv.n_bits(), 8);
        assert_eq!(bv.data.len(), 1);
        assert_eq!(bv.data[0], 0b11111111);
    }


    #[test]
    fn test_add_assign_triggers_resize() {
        let mut bv = BitVector::new();
        for _ in 0..8 {
            bv += false;
        }
        assert_eq!(bv.data.len(), 1);
        bv += true; // 9th bit, triggers resize
        assert_eq!(bv.n_bits(), 9);
        assert_eq!(bv.data.len(), 2);
        assert_eq!(bv.data[0], 0b00000000);
        assert_eq!(bv.data[1], 0b10000000);
    }

    #[test]
    fn test_add_assign_multiple_resizes() {
        let mut bv = BitVector::new();
        for i in 0..100 {
            bv += i % 2 == 0;
        }
        assert_eq!(bv.n_bits(), 100);
        assert!(bv.data.len() > 13);
        assert_eq!(bv.data[0], 0b10101010);
        assert_eq!(bv.data[12], 0b10100000);
    }

    #[test]
    fn test_add_assign_only_false() {
        let mut bv = BitVector::new();
        for _ in 0..16 {
            bv += false;
        }
        assert_eq!(bv.n_bits(), 16);
        assert_eq!(bv.data, vec![0b00000000, 0b00000000]);
    }

    #[test]
    fn test_add_assign_only_true() {
        let mut bv = BitVector::new();
        for _ in 0..16 {
            bv += true;
        }
        assert_eq!(bv.n_bits(), 16);
        assert_eq!(bv.data, vec![0b11111111, 0b11111111]);
    }

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
    fn test_equal_with_unused_bits_ignored() {
        // 10 bits = 1 full byte + 2 bits in next byte
        let mut bv1 = BitVector::with_bits(10);
        let mut bv2 = BitVector::with_bits(10);

        // Set same bits in both
        bv1.data[0] = 0b10101010;
        bv2.data[0] = 0b10101010;

        bv1.data[1] = 0b00000011; // only lower 2 bits used
        bv2.data[1] = 0b11111111; // bits beyond 2nd bit differ, but ignored

        assert_eq!(bv1, bv2);
    }

    #[test]
    fn test_not_equal_due_to_used_bits_difference() {
        // 10 bits again
        let mut bv1 = BitVector::with_bits(10);
        let mut bv2 = BitVector::with_bits(10);

        bv1.data[0] = 0b10101010;
        bv2.data[0] = 0b10101010;

        bv1.data[1] = 0b00000011;
        bv2.data[1] = 0b00000010; // difference in the 2nd bit (used bit)

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
    fn test_get_empty_vector() {
        let bv = BitVector::new();
        // Accessing any bit should panic because n_bits == 0
        let result = std::panic::catch_unwind(|| bv.get(0));
        assert!(result.is_err());
    }

    #[test]
    fn test_get_single_bit_set() {
        let mut bv = BitVector::with_bits(8);

        for i in 0..8 {
            assert_eq!(bv.get(i), false);
        }

        // Set bit 3 manually
        bv.data[0] |= 1 << 3;
        assert_eq!(bv.get(3), true);

        // Other bits remain false
        for i in 0..8 {
            if i != 3 {
                assert_eq!(bv.get(i), false);
            }
        }
    }

    #[test]
    fn test_get_multiple_bytes() {
        // Create a BitVector with 16 bits (2 bytes)
        let mut bv = BitVector::with_bits(16);
        // Set bit 0 and bit 15
        bv.data[0] |= 1 << 0;   // First bit of first byte
        bv.data[1] |= 1 << 7;   // Last bit of second byte (bit 15)

        assert_eq!(bv.get(0), true);
        assert_eq!(bv.get(15), true);

        // Check some bits that should be false
        for i in 1..15 {
            assert_eq!(bv.get(i), false);
        }
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_get_out_of_bounds() {
        let bv = BitVector::with_bits(8);
        // Accessing bit beyond n_bits should panic
        bv.get(8);
    }

    #[test]
    fn test_set_bit_true() {
        let mut bv = BitVector::with_bits(16);
        bv.set(3, true);
        assert!(bv.get(3)); // bit 3 should be set
        // Check other bits are still false
        assert!(!bv.get(0));
        assert!(!bv.get(15));
    }

    #[test]
    fn test_set_bit_false() {
        let mut bv = BitVector::with_bits(16);
        bv.set(5, true);
        assert!(bv.get(5));
        bv.set(5, false);
        assert!(!bv.get(5)); // bit 5 should be cleared
    }

    #[test]
    fn test_set_multiple_bits() {
        let mut bv = BitVector::with_bits(10);
        bv.set(0, true);
        bv.set(9, true);
        assert!(bv.get(0));
        assert!(bv.get(9));
        // bits in between should be false
        for i in 1..9 {
            assert!(!bv.get(i));
        }
    }

    #[test]
    #[should_panic] // If your BitVector does not handle out-of-bounds gracefully
    fn test_set_out_of_bounds() {
        let mut bv = BitVector::with_bits(8);
        bv.set(10, true); // Should panic or handle error
    }

}
