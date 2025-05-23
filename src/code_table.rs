use crate::bitvector::v1::BitVector;

struct Range {
    left: usize,
    right: usize,
}

struct Row {
    code: i32,
    n_bits: usize,
    range: Range,
}

struct CodeTable {
    lz_length: [Row; 29],
    lz_distance: [Row; 30],
    hff_length: [BitVector; 288],
    hff_distance: [BitVector; 32],
}