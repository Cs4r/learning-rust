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
            let n_bytes = n_bytes(n_bits);
            let vec : Vec<u8> =  vec![0; n_bytes as usize];
            
            BitVector {
                bits: vec,
                n_bits,
            }
        }
    }

    pub fn n_bits(&self) -> u64 {
        self.n_bits
    }

    pub fn n_bytes(&self) -> u64 {
        n_bytes(self.n_bits)
    }

}

impl Default for BitVector {
    fn default() -> Self {
        Self::new(0)
    }
}

fn n_bytes(n_bits: u64) -> u64 {
    (n_bits + 7) >> 3
}
