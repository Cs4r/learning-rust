struct BitVector {
    data: Vec<u8>,
    n_bits: usize,
}

impl BitVector {
    pub fn new() -> BitVector {
        BitVector {
            data: vec![0; 0],
            n_bits: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod constructors {
        use super::*;

        #[test]
        fn test_new_creates_empty_vector() {
            let bv = BitVector::new();
            assert_eq!(bv.data.len(), 0);
            assert_eq!(bv.n_bits, 0);
        }
    }
}
