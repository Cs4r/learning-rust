use crate::crc32::Crc32;
use std::error::Error;
use std::io::BufRead;

struct Lz77 {
    vector: Vec<u8>,
    index: isize,
    distance: i32,
    crc: Crc32,
}

impl Lz77 {
    pub fn new() -> Lz77 {
        Lz77 {
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

    pub fn from_reader<R: BufRead>(mut reader: R) -> Result<Self, Box<dyn Error>> {
        let mut lz77 = Lz77::new();
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer)?;

        let s = String::from_utf8(buffer)?;

        for ch in s.bytes() {
            lz77.add(ch);
        }

        Ok(lz77)
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

    mod from_reader_behavior {
        use super::*;
        use std::io::{Cursor, Read};
        #[test]
        fn test_from_reader_ascii() {
            let input = "hola mundo";
            let cursor = Cursor::new(input.as_bytes());

            let lz77 = Lz77::from_reader(cursor).expect("Failed to read");

            assert_eq!(lz77.vector.len(), input.bytes().count());

            let collected: String = lz77.vector.iter().map(|c| *c as char).collect();

            assert_eq!(collected, input);
        }

        #[test]
        fn test_from_reader_empty() {
            let input = "";
            let cursor = Cursor::new(input.as_bytes());

            let lz77 = Lz77::from_reader(cursor).expect("Failed to read");

            assert_eq!(lz77.vector.len(), 0);
        }
        #[test]
        fn test_from_reader_unicode() {
            let input = "¡Hola, mundo! 😊";
            let cursor = Cursor::new(input.as_bytes());

            let lz77 = Lz77::from_reader(cursor).expect("Failed to read");

            assert_eq!(lz77.vector.len(), input.bytes().count());
            let collected = String::from_utf8(lz77.vector).unwrap();
            
            assert_eq!(collected, input);
        }
    }
}
