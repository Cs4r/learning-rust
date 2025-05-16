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
            let vec : Vec<u8> =  vec![0; n_bytes];
            
            BitVector {
                data: vec,
                n_bits,
            }
        }
    }

    pub fn n_bits(&self) -> usize {
        self.n_bits
    }

    pub fn n_bytes(&self) -> usize {
        n_bytes(self.n_bits)
    }

}

fn n_bytes(n_bits: usize) -> usize {
    (n_bits + 7) >> 3
}