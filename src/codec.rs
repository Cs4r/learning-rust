use crate::bitvector::v1::BitVector;
use std::collections::HashMap;
use std::rc::Rc;

struct Codec {
    by_byte: HashMap<u8, Rc<BitVector>>,
    by_vector: HashMap<Rc<BitVector>, u8>,
}

impl Codec {
    pub fn new() -> Self {
        Codec {
            by_byte: HashMap::new(),
            by_vector: HashMap::new(),
        }
    }

    pub fn register_code(&mut self, byte: u8, bits: BitVector) {
        let shared = Rc::new(bits);
        self.by_vector.insert(Rc::clone(&shared), byte);
        self.by_byte.insert(byte, shared);
    }

    pub fn is_byte_encoded(&self, byte: u8) -> bool {
        self.by_byte.contains_key(&byte)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod constructor {
        use super::*;
        #[test]
        fn test_new_creates_empty_codec() {
            let codec = Codec::new();
            assert_eq!(codec.by_byte.len(), 0);
            assert_eq!(codec.by_vector.len(), 0);
        }
    }

    mod register_code_behavior {
        use super::*;

        #[test]
        fn test_register_code_stores_mappings() {
            let mut codec = Codec::new();

            let bv1: BitVector = "10".parse().unwrap();

            let bv2: BitVector = "01".parse().unwrap();

            let byte_a = 42;
            let byte_b = 7;

            codec.register_code(byte_a, bv1);
            codec.register_code(byte_b, bv2);

            assert!(codec.by_byte.contains_key(&byte_a));
            assert!(codec.by_byte.contains_key(&byte_b));
            assert!(!codec.by_byte.contains_key(&100));

            let bv_for_42 = codec.by_byte.get(&byte_a).unwrap();
            assert_eq!(codec.by_vector.get(bv_for_42), Some(&byte_a));

            let bv_for_7 = codec.by_byte.get(&byte_b).unwrap();
            assert_eq!(codec.by_vector.get(bv_for_7), Some(&byte_b));
        }

        #[test]
        fn test_register_code_overwrites_existing_mapping_for_same_byte() {
            let mut codec = Codec::new();

            let bit_vector_one: BitVector = "1".parse().unwrap();
            let bit_vector_zero: BitVector = "0".parse().unwrap();

            let byte_key = 10u8;

            codec.register_code(byte_key, bit_vector_one);
            codec.register_code(byte_key, bit_vector_zero);

            let stored_bit_vector = codec.by_byte.get(&byte_key).unwrap();

            assert_eq!(codec.by_vector.get(stored_bit_vector), Some(&byte_key));
        }
    }

    mod is_byte_encoded_behavior {
        use super::*;

        #[test]
        fn returns_false_on_empty_codec() {
            let empty_codec = Codec::new();

            assert!(!empty_codec.is_byte_encoded(4));
        }

        #[test]
        fn returns_true_when_byte_registered() {
            let mut codec = Codec::new();

            let byte = 55;
            codec.register_code(byte, BitVector::new());

            assert!(codec.is_byte_encoded(byte));
        }

        #[test]
        fn returns_false_when_byte_not_registered()  {
            let mut codec = Codec::new();

            let byte = 55;
            codec.register_code(byte, BitVector::new());

            assert!(!codec.is_byte_encoded(byte + 1));
        }

        #[test]
        fn returns_true_for_multiple_registered_bytes() {
            let mut codec = Codec::new();

            for byte in 0..10 {
                let bv: BitVector = if byte % 2 == 0 { "1".parse().unwrap() } else { "0".parse().unwrap() };
                codec.register_code(byte, bv);
            }

            for byte in 0..10 {
                assert!(codec.is_byte_encoded(byte));
            }
        }

        #[test]
        fn returns_false_for_unregistered_byte_among_registered_ones() {
            let mut codec = Codec::new();

            for byte in 0..10 {
                let bv: BitVector = "1".parse().unwrap();
                codec.register_code(byte, bv);
            }

            let unregistered_byte = 100;

            assert!(!codec.is_byte_encoded(unregistered_byte));
        }

    }
}
