use crate::crc32::Crc32;

struct Lz77 {
    vector: Vec<char>,
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

}