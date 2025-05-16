struct BitVector {
    bits: Vec<u8>,
    n_bits: u64,
}

impl BitVector {
    pub fn empty() -> BitVector {
        Self::new(0)
    }

    pub fn new(n_bits: u64) -> BitVector {
        if n_bits == 0 {
            BitVector {
                bits: vec![],
                n_bits: 0,
            }
        } else {
            let n_bytes = Self::n_bytes(n_bits);
            let vec : Vec<u8> =  vec![0; n_bytes as usize];
            
            BitVector {
                bits: vec,
                n_bits,
            }
        }
    }

    fn n_bytes(n_bits: u64) -> u64 {
        (n_bits + 7) >> 3
    }
}


