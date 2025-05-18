use crate::crc32::Crc32;

struct Lz77 {
    vector: Vec<u8>,
    index: isize,
    distance: i32,
    crc: Crc32,
}



impl Lz77 {

    pub fn new() -> Lz77 {
        Lz77{
            vector: vec![],
            index: 0,
            distance: 0,
            crc: Crc32::new(),
        }
    }
    
    pub fn add(&mut self, byte: u8) {
        self.vector.push(byte);
        self.crc.add(byte);
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    mod constructors {
        use super::*;

        #[test]
        fn test_new() {
            let lz77 = Lz77::new();

            assert!(lz77.vector.is_empty());
            assert_eq!(0, lz77.index);
            assert_eq!(0, lz77.distance);
            assert!(lz77.crc == Crc32::new());
        }
    }

    mod add_behavior {
        use super::*;

        #[test]
        fn test_add() {
            let mut lz77 = Lz77::new();
            lz77.add(128);

            assert_eq!(lz77.vector.len(), 1);
            assert_eq!(lz77.vector[0], 128);
            assert_eq!(lz77.distance, 0);
            assert_eq!(lz77.index, 0);
            assert_ne!(lz77.crc.get(), 0xFFFF_FFFF);
        }
    }

}