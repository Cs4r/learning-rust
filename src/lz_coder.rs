use std::error::Error;
use std::io::BufRead;

pub struct LzCoder {
    vector: Vec<char>,
    index: isize,
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

    pub fn compressed_string(&mut self) -> String {
        let mut output = String::new();
        let mut next = self.next();

        while next != 256 {
            if next < 256 {
                output.push(char::from_u32(next as u32).unwrap());
            } else {
                output.push_str(&format!("#REF({},{})#", next - 256, self.distance));
            }
            next = self.next();
        }

        output
    }

    fn next(&mut self) -> i32 {
        self.distance = 0;
        let mut cursor = self.index;
        let mut i = self.index - 1;
        let tam = self.vector.len() as isize;
        let mut length: isize = 0;
        let mut keep_searching = true;

        while i >= 0 && cursor < tam && keep_searching {
            if self.vector[i as usize] == self.vector[cursor as usize] {
                cursor += 1;
                i += 1;
                length += 1;
                self.distance = (self.index - (i - length)) as i32;

                if length == 258 || self.distance > 32768 {
                    keep_searching = false;
                }
            } else if length < 3 {
                cursor = self.index;
                i = i - length - 1;
                length = 0;
                self.distance = 0;
            } else {
                keep_searching = false;
            }
        }

        if length >= 3 && self.distance <= 32768 {
            self.index = cursor;
            (length + 256) as i32
        } else {
            if self.index < tam {
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
        use super::*;
        use std::io::Cursor;
        #[test]
        fn test_from_reader_ascii() {
            let input = "hola mundo";
            let cursor = Cursor::new(input.as_bytes());

            let lz_coder = LzCoder::from_reader(cursor).expect("Failed to read");

            assert_eq!(lz_coder.vector.len(), input.chars().count());

            let collected: String = lz_coder.vector.iter().collect();
            assert_eq!(collected, input);
        }

        #[test]
        fn test_from_reader_unicode() {
            let input = "¡Hola, mundo! 😊";
            let cursor = Cursor::new(input.as_bytes());

            let lz_coder = LzCoder::from_reader(cursor).expect("Failed to read");

            assert_eq!(lz_coder.vector.len(), input.chars().count());

            let collected: String = lz_coder.vector.iter().collect();
            assert_eq!(collected, input);
        }

        #[test]
        fn test_from_reader_empty() {
            let input = "";
            let cursor = Cursor::new(input.as_bytes());

            let lz_coder = LzCoder::from_reader(cursor).expect("Failed to read");

            assert_eq!(lz_coder.vector.len(), 0);
        }
    }

    mod compressed_string_behaviour {
        use crate::lz_coder::LzCoder;
        use std::io::Cursor;

        #[test]
        fn test_compressed_string_sample1() {
            test_compressed_string("There is a cow. The cow is white. The cow is big. The cow is a mammal.",
                                   "There is a cow. #REF(3,16)##REF(4,9)##REF(4,18)#white#REF(13,18)#big#REF(13,16)#a mammal.");
        }

        #[test]
        fn test_compressed_string_sample2() {
            test_compressed_string("The quick brown fox jumps over the lazy dog. The quick brown fox is fast.", "The quick brown fox jumps over t#REF(3,31)#lazy dog. #REF(20,45)#is fast.")
        }

        #[test]
        fn test_compressed_string_sample3() {
            test_compressed_string("Hello world! Hello world! Hello everyone.", "Hello world! #REF(19,13)#everyone.")
        }

        #[test]
        fn test_compressed_string_sample4() {
            test_compressed_string("There is a cat. The cat is small. The cat is quiet.", "There is a cat. #REF(3,16)##REF(4,9)##REF(4,18)#small#REF(13,18)#quiet.")
        }

        fn test_compressed_string(input: &str, expected_output: &str) {
            let cursor = Cursor::new(input.as_bytes());
            let mut lz_coder = LzCoder::from_reader(cursor).expect("Failed to read");

            let output = lz_coder.compressed_string();
            assert_eq!(output, expected_output);
        }
    }
}
