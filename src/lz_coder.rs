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
}