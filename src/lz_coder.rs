use std::error::Error;
use std::io::BufRead;

pub struct LzCoder {
    vector: Vec<char>,
    index: usize,
    distance: i32,
}


impl LzCoder {

    pub fn new() -> LzCoder {
        LzCoder {
            vector: Vec::new(),
            index: 0,
            distance: 0,
        }
    }

    pub fn from_reader<R: BufRead>(mut reader: R) -> Result<Self, Box<dyn Error>> {
        let mut lz_coder = LzCoder::new();
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer)?;

        let s = String::from_utf8(buffer)?;

        for ch in s.chars() {
            lz_coder.vector.push(ch);
        }

        Ok(lz_coder)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    mod constructor {
        use super::*;
        #[test]
        fn test_new_creates_empty_coder() {
            let coder = LzCoder::new();
            assert_eq!(coder.vector.len(), 0);
            assert_eq!(coder.index, 0);
            assert_eq!(coder.distance, 0);
        }
    }

    mod from_reader_behaviour {
        use std::io::Cursor;
        use super::*;
        #[test]
        fn test_from_reader_ascii() {
            let input = "hola mundo";
            let cursor = Cursor::new(input.as_bytes());

            let lz = LzCoder::from_reader(cursor).expect("Failed to read");

            assert_eq!(lz.vector.len(), input.chars().count());

            let collected: String = lz.vector.iter().collect();
            assert_eq!(collected, input);
        }

        #[test]
        fn test_from_reader_unicode() {
            let input = "¡Hola, mundo! 😊";
            let cursor = Cursor::new(input.as_bytes());

            let lz = LzCoder::from_reader(cursor).expect("Failed to read");

            assert_eq!(lz.vector.len(), input.chars().count());

            let collected: String = lz.vector.iter().collect();
            assert_eq!(collected, input);
        }

        #[test]
        fn test_from_reader_empty() {
            let input = "";
            let cursor = Cursor::new(input.as_bytes());

            let lz = LzCoder::from_reader(cursor).expect("Failed to read");

            assert_eq!(lz.vector.len(), 0);
        }
    }
}