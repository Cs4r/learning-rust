use crate::crc32::Crc32;

struct Lz77 {
    vector: Vec<char>,
    index: isize,
    distance: i32,
    crc: Crc32,
}

