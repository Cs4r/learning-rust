use std::collections::HashMap;
use crate::bitvector::v1::BitVector;

struct Codec {
    by_byte: HashMap<u8, BitVector>,
    by_vector : HashMap<BitVector, u8>,
}

impl Codec {
    fn new() -> Self {
        Codec {
            by_byte: HashMap::new(),
            by_vector: HashMap::new(),
        }
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
}