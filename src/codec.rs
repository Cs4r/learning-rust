use std::collections::HashMap;
use crate::bitvector::v1::BitVector;

struct Codec {
    by_byte: HashMap<u8, BitVector>,
    by_vector : HashMap<BitVector, u8>,
}