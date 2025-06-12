use crate::crc32::Crc32;
use std::error::Error;
use std::io::BufRead;


#[derive(Default)]
pub struct Lz77 {
    vector: Vec<u8>,
    index: isize,
    distance: i32,
    crc: Crc32,
}

impl Lz77 {
    
    pub fn add(&mut self, byte: u8) {
        self.vector.push(byte);
        self.crc.add(byte);
    }

    pub fn from_reader<R: BufRead>(mut reader: R) -> Result<Self, Box<dyn Error>> {
        let mut lz77 = Lz77::default();

        for byte_result in reader.bytes() {
            let byte = byte_result?; // Manejo de error
            lz77.add(byte);
        }

        Ok(lz77)
    }

    pub fn get_distance(&self) -> i32 {
        self.distance
    }

    pub fn get_crc32(&mut self) -> u32 {
        self.crc.get()
    }

    pub fn get_size(&self) -> usize {
        self.vector.len()
    }

    pub fn next(&mut self) -> i32 {
        self.distance = 0;
        let len = self.vector.len() as isize;
        let mut cursor = self.index;
        let mut search_pos = self.index - 1;
        let mut match_length: isize = 0;

        while search_pos >= 0 && cursor < len {
            if self.vector[search_pos as usize] == self.vector[cursor as usize] {
                // Characters match, extend match
                cursor += 1;
                search_pos += 1;
                match_length += 1;
                // Update distance: distance = current index - start position of the match in vector
                self.distance = (self.index - (search_pos - match_length)) as i32;

                // Stop if match is max length or distance too large
                if match_length == 258 || self.distance > 32768 {
                    break;
                }
            } else if match_length < 3 {
                // Match too short, reset cursor and move search position one step backward
                cursor = self.index;
                search_pos = search_pos - match_length - 1;
                match_length = 0;
                self.distance = 0;
            } else {
                // Found a match >= 3, but current characters don't match, so stop searching
                break;
            }
        }

        // If a sufficiently long match found with valid distance
        if match_length >= 3 && self.distance <= 32768 {
            self.index = cursor;
            (match_length + 256) as i32 // Encoded length marker
        } else {
            // No match found, output literal byte or end marker (256)
            if self.index < len {
                let ch = self.vector[self.index as usize] as i32;
                self.index += 1;
                ch
            } else {
                256
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod constructors {
        use super::*;

        #[test]
        fn test_default() {
            let lz77 = Lz77::default();

            assert!(lz77.vector.is_empty());
            assert_eq!(0, lz77.index);
            assert_eq!(0, lz77.distance);
            assert!(lz77.crc == Crc32::default());
        }
    }

    mod add_behavior {
        use super::*;

        #[test]
        fn test_add() {
            let mut lz77 = Lz77::default();
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
