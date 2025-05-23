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
    pub fn new() -> Self {
        // Initialize the LZ77 length table with default values.
        let mut lz_length = [Row {
            code: 0,
            n_bits: 0,
            range: Range { left: 0, right: 0 },
        }; 29];

        // The first length code (257) corresponds to a length of 3.
        lz_length[0] = Row {
            code: 257,
            n_bits: 0,
            range: Range { left: 3, right: 3 },
        };

        // Fill the next 7 entries with range increasing by 1 (lengths 4 to 10).
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

        // Starting from length 11, the number of extra bits increases.
        let mut num_bits = 1;
        let mut amplitude = 2;
        let mut i = 8;
        while i < 27 {
            // The range increases exponentially depending on the amplitude.
            let left = lz_length[i - 1].range.right + 1;
            let right = left + amplitude - 1;
            lz_length[i] = Row {
                code: lz_length[i - 1].code + 1,
                n_bits: num_bits,
                range: Range { left, right },
            };
            i += 1;
            // Every few steps, the number of bits and amplitude doubles.
            if i == 12 || i == 16 || i == 20 || i == 24 || i == 27 {
                num_bits += 1;
                amplitude *= 2;
            }
        }

        // Last two length codes (284 and 285) are manually added.
        lz_length[27] = Row {
            code: 284,
            n_bits: 5,
            range: Range {
                left: 227,
                right: 257,
            },
        };
        lz_length[28] = Row {
            code: 285,
            n_bits: 0,
            range: Range {
                left: 258,
                right: 258,
            },
        };

        // Initialize the LZ77 distance table with default values.
        let mut lz_distance = [Row {
            code: 0,
            n_bits: 0,
            range: Range { left: 0, right: 0 },
        }; 30];

        // First distance code corresponds to distance 1.
        lz_distance[0] = Row {
            code: 0,
            n_bits: 0,
            range: Range { left: 1, right: 1 },
        };

        // Next 3 codes correspond to distances 2, 3, and 4.
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

        // Starting from distance 5, use extra bits and increasing range.
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
            // Increase number of bits and amplitude every 2 entries.
            if i % 2 == 0 {
                num_bits += 1;
                amplitude *= 2;
            }
        }

        // Huffman fixed codes for distance (5 bits each).
        let hff_distance: [BitVector; 32] =
            std::array::from_fn(|i| BitVector::from_value(i as u32, 5));

        // Huffman fixed codes for literal/length values.
        let hff_length: [BitVector; 288] = std::array::from_fn(|i| {
            let i = i as u32;
            if i < 144 {
                // Codes 0..143: 8-bit codes starting at 0b00110000
                BitVector::from_value(i + 48, 8)
            } else if i < 256 {
                // Codes 144..255: 9-bit codes starting at 0b110010000
                BitVector::from_value(256 + i, 9)
            } else if i < 280 {
                // Codes 256..279: 7-bit codes starting at 0b0000000
                BitVector::from_value(i - 256, 7)
            } else {
                // Codes 280..287: 8-bit codes starting at 0b11000000
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


