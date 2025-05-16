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

        for i in 0..self.n_bytes() {
            if self.data[i] != other.data[i] {
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

}
