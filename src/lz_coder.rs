use std::error::Error;
use std::io::BufRead;


#[derive(Default)]
pub struct LzCoder {
    vector: Vec<char>,
    index: isize,
    distance: i32,
}

impl LzCoder {
    
    pub fn from_reader<R: BufRead>(mut reader: R) -> Result<Self, Box<dyn Error>> {
        let mut lz_coder = LzCoder::default();
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer)?;

        let s = String::from_utf8(buffer)?;

        for ch in s.chars() {
            lz_coder.vector.push(ch);
        }

        Ok(lz_coder)
    }

    pub fn get_distance(&self) -> i32 {
        self.distance
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
    mod constructor {
        use super::*;
        #[test]
        fn test_default_creates_empty_coder() {
            let coder = LzCoder::default();
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
}
