use crate::bitvector::v2::BitVector;

#[derive(Clone, Copy)]
struct Range {
    left: usize,
    right: usize,
}

#[derive(Clone, Copy)]
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
impl CodeTable {
    fn new() -> Self {
        let mut lz_length = [Row {
            code: 0,
            n_bits: 0,
            range: Range { left: 0, right: 0 },
        }; 29];

        lz_length[0] = Row {
            code: 257,
            n_bits: 0,
            range: Range { left: 3, right: 3 },
        };

        for i in 1..8 {
            lz_length[i] = Row {
                code: lz_length[i - 1].code + 1,
                n_bits: 0,
                range: Range {
                    left: lz_length[i - 1].range.left + 1,
                    right: lz_length[i - 1].range.right + 1,
                },
            };
        }

        let mut num_bits = 1;
        let mut amplitude = 2;
        let mut i = 8;
        while i < 27 {
            let left = lz_length[i - 1].range.right + 1;
            let right = left + amplitude - 1;
            lz_length[i] = Row {
                code: lz_length[i - 1].code + 1,
                n_bits: num_bits,
                range: Range { left, right },
            };
            i += 1;
            if i == 12 || i == 16 || i == 20 || i == 24 || i == 27 {
                num_bits += 1;
                amplitude *= 2;
            }
        }

        lz_length[27] = Row {
            code: 284,
            n_bits: 5,
            range: Range { left: 227, right: 257 },
        };
        lz_length[28] = Row {
            code: 285,
            n_bits: 0,
            range: Range { left: 258, right: 258 },
        };

        let mut lz_distance = [Row {
            code: 0,
            n_bits: 0,
            range: Range { left: 0, right: 0 },
        }; 30];

        lz_distance[0] = Row {
            code: 0,
            n_bits: 0,
            range: Range { left: 1, right: 1 },
        };

        for i in 1..4 {
            lz_distance[i] = Row {
                code: lz_distance[i - 1].code + 1,
                n_bits: 0,
                range: Range {
                    left: lz_distance[i - 1].range.right + 1,
                    right: lz_distance[i - 1].range.right + 1,
                },
            };
        }

        let mut i = 4;
        num_bits = 1;
        amplitude = 2;
        while i < 30 {
            let left = lz_distance[i - 1].range.right + 1;
            let right = left + amplitude - 1;
            lz_distance[i] = Row {
                code: lz_distance[i - 1].code + 1,
                n_bits: num_bits,
                range: Range { left, right },
            };
            i += 1;
            if i % 2 == 0 {
                num_bits += 1;
                amplitude *= 2;
            }
        }
        
        let hff_distance: [BitVector; 32] = std::array::from_fn(|i| BitVector::from_value(i, 5));

        let hff_length: [BitVector; 288] = std::array::from_fn(|i| {
            if i < 144 {
                BitVector::from_value(i + 48, 8)
            } else if i < 256 {
                BitVector::from_value(256 + i, 9)
            } else if i < 280 {
                BitVector::from_value(i - 256, 7)
            } else {
                BitVector::from_value(i - 88, 8)
            }
        });

        Self {
            lz_length,
            lz_distance,
            hff_length,
            hff_distance,
        }
    }
}