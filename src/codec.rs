use crate::bitvector::v1::BitVector;
use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;
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

    pub fn is_bitvector_encoded(&self, bit_vector: &BitVector) -> bool {
        self.by_vector.contains_key(bit_vector)
    }

    pub fn get_byte(&self, bit_vector: &BitVector) -> u8 {
        *self.by_vector.get(bit_vector).unwrap()
    }

    pub fn get_bitvector(&self, byte: u8) -> &BitVector {
        self.by_byte.get(&byte).unwrap()
    }


    pub fn from_reader<R: BufRead>(mut reader: R) -> Result<Self, Box<dyn Error>> {
        let mut first_line = String::new();
        reader.read_line(&mut first_line)?;
        let n: usize = first_line.trim().parse()?;

        let mut codec = Codec::new();

        for _ in 0..n {
            let mut char_buf = [0u8; 1];
            reader.read_exact(&mut char_buf)?;
            let byte = char_buf[0];

            let mut space_buf = [0u8; 1];
            reader.read_exact(&mut space_buf)?;
            if space_buf[0] != b' ' {
                return Err("Expected space after character".into());
            }

            let mut bv_line = String::new();
            reader.read_line(&mut bv_line)?;
            let bit_str = bv_line.trim_end_matches(&['\r', '\n'][..]);

            let bv: BitVector = bit_str.parse()?;
            codec.register_code(byte, bv);
        }

        Ok(codec)
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

    mod is_bitvector_encoded_behavior {
        use super::*;

        #[test]
        fn returns_false_when_map_is_empty() {
            let codec = Codec::new();

            let bv: BitVector = "1".parse().unwrap();

            assert!(!codec.is_bitvector_encoded(&bv));
        }

        #[test]
        fn returns_true_after_registering_bitvector() {
            let mut codec = Codec::new();
            let bv: BitVector = "101".parse().unwrap();

            codec.register_code(10, bv.clone());

            assert!(codec.is_bitvector_encoded(&bv));
        }

        #[test]
        fn returns_false_for_unregistered_bitvector() {
            let codec = Codec::new();
            let bv: BitVector = "101".parse().unwrap();

            assert!(!codec.is_bitvector_encoded(&bv));
        }

        #[test]
        fn returns_true_for_multiple_registered_bitvectors() {
            let mut codec = Codec::new();

            let bit_strings = ["1", "10", "101", "111"];
            for (i, bits) in bit_strings.iter().enumerate() {
                let bv: BitVector = bits.parse().unwrap();
                codec.register_code(i as u8, bv);
            }

            for bits in &bit_strings {
                let bv: BitVector = bits.parse().unwrap();
                assert!(codec.is_bitvector_encoded(&bv));
            }
        }

        #[test]
        fn returns_true_for_different_instances_with_same_bits() {
            let mut codec = Codec::new();

            let bv1: BitVector = "101".parse().unwrap();
            let bv2: BitVector = "101".parse().unwrap();

            codec.register_code(42, bv1);

            assert!(codec.is_bitvector_encoded(&bv2));
        }
    }

    mod get_byte_behavior {
        use super::*;

        #[test]
        fn returns_registered_byte_for_bitvector() {
            let mut codec = Codec::new();
            let bv: BitVector = "101".parse().unwrap();
            let byte = 42;

            codec.register_code(byte, bv.clone());

            assert_eq!(codec.get_byte(&bv), byte);
        }

        #[test]
        #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
        fn panics_if_bitvector_not_registered() {
            let codec = Codec::new();
            let bv: BitVector = "101".parse().unwrap();

            codec.get_byte(&bv);
        }

        #[test]
        fn returns_correct_byte_for_multiplne_registered_bitvectors() {
            let mut codec = Codec::new();

            let pairs = [("1", 1), ("10", 2), ("101", 5), ("111", 7)];

            for (bits, byte) in pairs.iter() {
                let bv: BitVector = bits.parse().unwrap();
                codec.register_code(*byte, bv);
            }

            for (bits, byte) in pairs.iter() {
                let bv: BitVector = bits.parse().unwrap();
                assert_eq!(codec.get_byte(&bv), *byte);
            }
        }
    }

    mod get_bitvector_behavior {
        use super::*;

        #[test]
        fn returns_correct_bitvector_after_registering() {
            let mut codec = Codec::new();
            let bv: BitVector = "101".parse().unwrap();

            codec.register_code(10, bv.clone());

            let retrieved = codec.get_bitvector(10);
            assert_eq!(retrieved, &bv);
        }

        #[test]
        #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
        fn panics_when_byte_not_registered() {
            let codec = Codec::new();
            // No registration of byte 42

            codec.get_bitvector(42); // Should panic because 42 not registered
        }

        #[test]
        fn works_with_multiple_registered_bitvectors() {
            let mut codec = Codec::new();

            let pairs = [
                (1, "0"),
                (2, "1"),
                (3, "11"),
            ];

            for (byte, bits) in pairs.iter() {
                let bv: BitVector = bits.parse().unwrap();
                codec.register_code(*byte, bv);
            }

            for (byte, bits) in pairs.iter() {
                let expected: BitVector = bits.parse().unwrap();
                let actual = codec.get_bitvector(*byte);
                assert_eq!(actual, &expected);
            }
        }
    }

    mod from_reader_behavior {
        use super::*;
        use std::io::Cursor;

        #[test]
        fn parses_correctly() {
            let data = "2\na 101\nb 010\n";

            let cursor = Cursor::new(data);
            let codec = Codec::from_reader(cursor).expect("Should parse correctly");

            assert!(codec.is_byte_encoded(b'a'));
            assert!(codec.is_byte_encoded(b'b'));

            let bv_a = codec.get_bitvector(b'a');
            assert_eq!(bv_a.to_string(), "101");

            let bv_b = codec.get_bitvector(b'b');
            assert_eq!(bv_b.to_string(), "010");
        }

        #[test]
        fn handles_newline_as_byte() {
            let data = "1\n\n 1010\n";
            let cursor = std::io::Cursor::new(data);
            let codec = Codec::from_reader(cursor).expect("Should parse newline byte");

            assert!(codec.is_byte_encoded(b'\n'));
            let bv_nl = codec.get_bitvector(b'\n');
            assert_eq!(bv_nl.to_string(), "1010");
        }

        #[test]
        fn returns_error_on_empty_file() {
            let data = "";
            let cursor = Cursor::new(data);

            let res = Codec::from_reader(cursor);
            assert!(res.is_err());
        }

        #[test]
        fn returns_error_on_invalid_number_of_lines() {
            let data = "not_a_number\n";
            let cursor = Cursor::new(data);

            let res = Codec::from_reader(cursor);
            assert!(res.is_err());
        }

        #[test]
        fn returns_error_on_unexpected_eof() {
            let data = "2\na 1\n";
            let cursor = Cursor::new(data);

            let res = Codec::from_reader(cursor);
            assert!(res.is_err());
        }

        #[test]
        fn returns_error_on_missing_bitvector() {
            let data = "1\na\n"; // only one token, no bitvector
            let cursor = Cursor::new(data);

            let res = Codec::from_reader(cursor);
            assert!(res.is_err());
        }

        #[test]
        fn returns_error_on_invalid_byte_length() {
            let data = "1\nab 101\n"; // more than one char in byte field
            let cursor = Cursor::new(data);

            let res = Codec::from_reader(cursor);
            assert!(res.is_err());
        }
    }

}
