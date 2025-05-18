
struct BitVector {
    data: Vec<u8>,
    n_bits: usize
}


impl BitVector {
    pub fn new(n_bits: usize) -> BitVector {
        BitVector {
            data: vec![0; 0],
            n_bits
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod constructors {
        use super::*;

        #[test]
        fn test_constructors() {

        }
    }
}