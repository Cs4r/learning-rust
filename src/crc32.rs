use std::sync::OnceLock;

struct Crc32(u32);

impl Crc32 {
    pub fn new() -> Self {
        Crc32(0xFFFFFFFF)
    }

    pub fn get(&mut self) -> u32 {
        self.0 = self.0 ^ 0xFFFFFFFF;
        self.0
    }

    pub fn add(&mut self, byte: u8) {
        let index = ((self.0 as u8 ^ byte) & 0xFF) as usize;
        self.0 = (self.0 >> 8) ^ crc32_table()[index];
    }
}

pub fn crc32(input: &[u8]) -> u32 {
    let table = crc32_table();
    let mut crc32: u32 = 0xFFFFFFFF;

    for &byte in input {
        crc32 = (crc32 >> 8) ^ table[((crc32 ^ byte as u32) & 0xFF) as usize];
    }

    crc32 = crc32 ^ 0xFFFFFFFF;
    crc32
}

static CRC32_TABLE: OnceLock<[u32; 256]> = OnceLock::new();

fn crc32_table() -> &'static [u32; 256] {
    CRC32_TABLE.get_or_init(|| {
        let mut table = [0u32; 256];
        for n in 0..256 {
            let mut c = n as u32;
            for _ in 0..8 {
                if c & 1 != 0 {
                    c = 0xEDB88320 ^ (c >> 1);
                } else {
                    c >>= 1;
                }
            }
            table[n] = c;
        }
        table
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    mod constructor {
        use crate::crc32::Crc32;

        #[test]
        fn new_sets_all_bits_to_one() {
            let crc32 = Crc32::new();

            assert_eq!(crc32.0, 0xFFFFFFFF);
        }
    }
    
    mod get_and_add_behavior {
        use crate::crc32::{crc32, Crc32};

        #[test]
        fn test_crc32_hello_world() {
            let data = b"hello world";
            
            let mut crc32 = Crc32::new();
            
            for i in data {
                crc32.add(*i);
            }
            
            assert_eq!(crc32.get(), 0x0D4A1185);
        }

        #[test]
        fn test_crc32_empty() {
            let data = b"";

            let mut crc32 = Crc32::new();
            
            assert_eq!(crc32.get(), 0);
        }

        #[test]
        fn test_crc32_known_value() {
            let data = b"123456789";

            let mut crc32 = Crc32::new();
            
            for i in data {
                crc32.add(*i);
            }
            
            assert_eq!(crc32.get(), 0xCBF43926);
        }
    }

    mod crc32_function_behavior {
        use crate::crc32::crc32;
        #[test]
        fn test_crc32_empty() {
            let data = b"";
            assert_eq!(crc32(data), 0);
        }

        #[test]
        fn test_crc32_hello_world() {
            let data = b"hello world";
            assert_eq!(crc32(data), 0x0D4A1185);
        }

        #[test]
        fn test_crc32_known_value() {
            let data = b"123456789";
            assert_eq!(crc32(data), 0xCBF43926);
        }

        #[test]
        fn test_crc32_single_byte() {
            let data = b"a";
            assert_eq!(crc32(data), 0xE8B7BE43);
        }

        #[test]
        fn test_crc32_case_sensitive() {
            let data1 = b"ABC";
            let data2 = b"abc";
            assert_ne!(crc32(data1), crc32(data2));
        }
    }
}
